/* Copyright (c) 2019 University of Utah
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR(S) DISCLAIM ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL AUTHORS BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Distribution {
    Uniform,
    Zipf,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Isolation {
    NoIsolation,
    PageTableIsolation,
    MpkIsolation,
    VmfuncIsolation,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Policy {
    RoundRobin,
    Minos,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // The maximum number of small cores used for the simultaion.
    pub small_cores: u64,

    // The maximum number of large cores for the simulation.
    pub large_cores: u64,

    // The number of teants the client will generate requests for.
    pub num_tenants: u64,

    // Skew in picking the tenant for new request.
    pub tenant_skew: f64,

    // The number of requests that the client must generate.
    pub num_reqs: u64,

    // The number of responses that the client must receive before terminating the process.
    pub num_resps: u64,

    // The req rate per second.
    pub req_rate: u64,

    // Execute all the tasks for a tenant for each iteration.
    pub batching: bool,

    // This is to decide which memory isolation to use; No isolation, Paging, MPK, VMFunc.
    pub isolation: Isolation,

    // This is to decide which tenant distribution to use; Uniform Random or Zipfian.
    pub distribution: Distribution,

    // This is to decide the scheduling policy; Round Robin or minos like core partitioning.
    pub policy: Policy,
}

impl Config {
    pub fn load() -> Config {
        let mut contents = String::new();
        let filename = "config.toml";

        let _ = File::open(filename).and_then(|mut file| file.read_to_string(&mut contents));

        match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                panic!("Failure paring config file {}: {}", filename, e);
            }
        }
    }
}
