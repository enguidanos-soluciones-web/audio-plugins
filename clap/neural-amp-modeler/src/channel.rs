// Copyright (C) 2026 Cristian A. Enguídanos Nebot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crossbeam_queue::ArrayQueue;
use std::sync::Arc;

pub struct Sender<T>(Arc<ArrayQueue<T>>);
pub struct Receiver<T>(Arc<ArrayQueue<T>>);

pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    let q = Arc::new(ArrayQueue::new(capacity));
    (Sender(Arc::clone(&q)), Receiver(q))
}

impl<T> Sender<T> {
    pub fn push(&self, val: T) -> Result<(), T> {
        self.0.push(val)
    }

    pub fn new_receiver(&self) -> Receiver<T> {
        Receiver(Arc::clone(&self.0))
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender(Arc::clone(&self.0))
    }
}

impl<T> Receiver<T> {
    pub fn pop(&self) -> Option<T> {
        self.0.pop()
    }

    pub fn new_sender(&self) -> Sender<T> {
        Sender(Arc::clone(&self.0))
    }
}
