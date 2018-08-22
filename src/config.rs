// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use cminer::{Config, Worker};

use super::worker::BlakeWorker;

#[derive(Clone, Copy)]
pub struct BlakeConfig {
    pub listening_port: u16,
    pub submitting_port: u16,
}

impl Config for BlakeConfig {
    fn listening_port(&self) -> u16 {
        self.listening_port
    }

    fn submitting_port(&self) -> u16 {
        self.submitting_port
    }

    fn worker(&self) -> Box<Worker> {
        Box::new(BlakeWorker::new())
    }
}
