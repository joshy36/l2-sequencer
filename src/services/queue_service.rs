use crate::models::{L2Transaction, Queue};
use alloy::providers::Provider;
use tokio::sync::{broadcast, mpsc, oneshot};

// Commands that can be sent to the queue
#[derive(Debug)]
pub enum QueueCommand {
    SubmitTransaction {
        transaction: L2Transaction,
        response: oneshot::Sender<Result<(), String>>,
    },
    InitiateBatch {
        response: oneshot::Sender<Result<(), String>>,
    },
}

#[derive(Clone)]
pub struct QueueHandle {
    command_tx: mpsc::Sender<QueueCommand>,
    feed_tx: broadcast::Sender<L2Transaction>,
}

impl QueueHandle {
    pub fn new(
        command_tx: mpsc::Sender<QueueCommand>,
        feed_tx: broadcast::Sender<L2Transaction>,
    ) -> Self {
        Self {
            command_tx,
            feed_tx,
        }
    }

    pub async fn submit_transaction(&self, transaction: L2Transaction) -> Result<(), String> {
        let (response_tx, response_rx) = oneshot::channel();

        let tx = transaction.clone();

        self.command_tx
            .send(QueueCommand::SubmitTransaction {
                transaction,
                response: response_tx,
            })
            .await
            .map_err(|e| e.to_string())?;

        let _ = self.feed_tx.send(tx);

        response_rx.await.map_err(|e| e.to_string())?
    }

    pub async fn batch_transactions(&self) -> Result<(), String> {
        let (response_tx, response_rx) = oneshot::channel();

        self.command_tx
            .send(QueueCommand::InitiateBatch {
                response: response_tx,
            })
            .await
            .map_err(|e| e.to_string())?;

        response_rx.await.map_err(|e| e.to_string())?
    }

    pub fn subscribe(&self) -> broadcast::Receiver<L2Transaction> {
        self.feed_tx.subscribe()
    }
}

pub struct QueueProcessor<T: Provider> {
    queue: Queue<T>,
    command_rx: mpsc::Receiver<QueueCommand>,
}

impl<T: Provider> QueueProcessor<T> {
    pub fn new(provider: T, command_rx: mpsc::Receiver<QueueCommand>) -> Self {
        Self {
            queue: Queue::new(provider),
            command_rx,
        }
    }

    pub async fn run(&mut self) {
        while let Some(cmd) = self.command_rx.recv().await {
            match cmd {
                QueueCommand::SubmitTransaction {
                    transaction,
                    response,
                } => {
                    self.queue.queue_transaction(&transaction);
                    let _ = response.send(Ok(()));
                    self.queue.print_queue_state();
                }
                QueueCommand::InitiateBatch { response } => {
                    let _ = self.queue.batch_transactions();
                    let _ = response.send(Ok(()));
                    println!("Batch sent!");
                    self.queue.print_queue_state();
                }
            }
        }
    }
}

pub fn setup_queue<T: Provider>(provider: T) -> (QueueHandle, QueueProcessor<T>) {
    let (command_tx, command_rx) = mpsc::channel(100);
    let (feed_tx, _feed_rx) = broadcast::channel(100);
    let handle = QueueHandle::new(command_tx, feed_tx);
    let processor = QueueProcessor::new(provider, command_rx);
    (handle, processor)
}
