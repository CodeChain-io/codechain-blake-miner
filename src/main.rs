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

extern crate byteorder;
#[macro_use]
extern crate clap;
extern crate codechain_miner as cminer;
extern crate crypto;
extern crate env_logger;
extern crate ethereum_types;
#[macro_use]
extern crate log;
extern crate rlp;

mod config;
mod worker;

use cminer::run;

use self::config::BlakeConfig;

fn get_options() -> Result<BlakeConfig, String> {
    let yaml = load_yaml!("./cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let listening_port: u16 =
        matches.value_of("listening port").unwrap().parse().map_err(|_| "Invalid listening port")?;
    let submitting_port: u16 =
        matches.value_of("submitting port").unwrap().parse().map_err(|_| "Invalid submitting port")?;

    Ok(BlakeConfig {
        listening_port,
        submitting_port,
    })
}

fn main() -> Result<(), String> {
    env_logger::init();
    let worker_config = get_options()?;

    run(worker_config);
    Ok(())
}
