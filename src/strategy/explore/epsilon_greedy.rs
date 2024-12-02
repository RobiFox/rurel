/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use rand::Rng;
use crate::mdp::{Agent, State};
use crate::strategy::explore::ExplorationStrategy;

pub struct EpsilonGreedy {
    explore: f32
}

impl Default for EpsilonGreedy {
    fn default() -> Self {
        EpsilonGreedy { explore: 0.9 }
    }
}

impl<S: State> ExplorationStrategy<S> for EpsilonGreedy {
    fn pick_action(&self, agent: &mut dyn Agent<S>) -> S::A {
        let random = rand::thread_rng().gen_range(0.0..=1.0);
        if random < self.explore {
            todo!()
        } else {
            agent.pick_random_action()
        }
    }
}