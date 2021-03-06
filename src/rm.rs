/*
 * Copyright © 2020 rusty-snake
 *
 * This file is part of fjp
 *
 * fjp is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * fjp is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{profile_path, utils::get_name1};
use clap::ArgMatches;
use log::{debug, error, trace};
use std::fs::remove_file;

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: rm");

    let profiles = cli.values_of("PROFILE_NAMES").unwrap();
    for profile in profiles {
        let profile = get_name1(profile);
        trace!("Deleting '{}'.", profile);
        remove_file(profile_path!(USER / &profile))
            .unwrap_or_else(|err| error!("Failed to delete '{}': {}", profile, err));
    }
}
