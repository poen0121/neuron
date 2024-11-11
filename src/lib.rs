use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Neuron {
    // ---- Neuron cell parameters ----
    pub x: usize,  // neuron x-coordinate
    pub y: usize,  // neuron y-coordinate
    pub z: usize,  // neuron z-coordinate
    pub ax: usize, // axon x-coordinate
    pub ay: usize, // axon y-coordinate
    pub az: usize, // axon z-coordinate
    pub nt: u32,  // neuron type ( 0 = Contact , 1 = Sensory , 2 = Motor )
    pub nrt: u32, // neurotransmitter type ( 0 = Inhibitory , 1 = Excitatory )
    pub ap: f64,  // accumulated potential
    pub tp: f64,  // threshold potential
    pub mp: f64,  // membrane potential
    pub fr: f64,  // firing rate

    // ---- Synaptic plasticity related parameters ----
    pub sw: f64,  // synaptic weight
    pub sst: f64, // synaptic strength threshold
    pub pr: f64,  // plasticity rate

    // ---- Dynamic parameters related to neuron activity ----
    pub arp: f64,  // absolute refractory period
    pub rrp: f64,  // relative refractory period

    // ---- Parameters related to synaptic connection formation ----
    pub ac: HashSet<(usize, usize, usize)>, // axonal connections
    pub dc: HashSet<(usize, usize, usize)>, // dendritic connections

    // ---- Biological regulatory factors ----
    pub nc: f64,  // neurotransmitter concentration

    // ---- Long-term adjustment and pruning ----
    pub ltp: f64, // long term potentiation factor
    pub ltd: f64, // long term depression factor
}

impl Neuron {
    pub const BASE_ABSOLUTE_REFRACTORY_PERIOD: f64 = 1.0;
    pub const ABSOLUTE_REFRACTORY_PERIOD_DECREASE_FACTOR: f64 = 0.99;
    pub const BASE_RELATIVE_REFRACTORY_PERIOD: f64 = 1.0;
    pub const RELATIVE_REFRACTORY_PERIOD_RECOVERY_FACTOR: f64 = 0.165;
    pub const RESTING_POTENTIAL: f64 = -70.0;
    pub const ACCUMULATED_POTENTIAL_CRITICAL_VALUE: f64 = 10.0;
    pub const ACCUMULATED_POTENTIAL_STIMULUS_INTENSITY : f64 = 0.8;
    pub const ACCUMULATED_POTENTIAL_SLIGHT_INTENSITY : f64 = 0.08;
    pub const MAX_THRESHOLD_POTENTIAL: f64 = -50.0;
    pub const MIN_THRESHOLD_POTENTIAL: f64 = -55.0;
    pub const THRESHOLD_POTENTIAL_BOOST_FACTOR_FOR_ACCUMULATED_POTENTIAL: f64 = 0.01;
    pub const THRESHOLD_POTENTIAL_BOOST_FACTOR_FOR_FIRING_RATE: f64 = 0.02;
    pub const MIN_MEMBRANE_POTENTIAL: f64 = -90.0;
    pub const MAX_MEMBRANE_POTENTIAL: f64 = -20.0;
    pub const MIN_EXCITATORY_SIGNAL: f64 = 1.0;
    pub const MAX_EXCITATORY_SIGNAL: f64 = 30.0;
    pub const MIN_INHIBITORY_SIGNAL: f64 = -20.0;
    pub const MAX_INHIBITORY_SIGNAL: f64 = -1.0;
    pub const MAX_FIRING_RATE: f64 = 1.0;
    pub const FIRING_RATE_DECREASE_FACTOR: f64 = 0.92;
    pub const FIRING_RATE_BOOST_FACTOR: f64 = 0.01;
    pub const MAX_PLASTICITY_RATE: f64 = 1.0;
    pub const PLASTICITY_RATE_DECREASE_FACTOR: f64 = 0.96;
    pub const PLASTICITY_RATE_BOOST_FACTOR: f64 = 0.01;
    pub const MAX_LTP: f64 = 1.0;
    pub const LTP_BOOST_FACTOR: f64 = 0.01;
    pub const LTP_DECREASE_FACTOR: f64 = 0.96;
    pub const MIN_LTD: f64 = -1.0;
    pub const LTD_BOOST_FACTOR: f64 = 0.01;
    pub const LTD_DECREASE_FACTOR: f64 = 0.96;
    pub const SYNAPTIC_STRENGTH_THRESHOLD_BOOST_FACTOR: f64 = 0.01;

    // Creates a new Neuron instance.
    // Parameters:
    // - `x`: Neuron x-coordinate.
    // - `y`: Neuron y-coordinate.
    // - `z`: Neuron z-coordinate.
    // - `ax`: Axon x-coordinate.
    // - `ay`: Axon y-coordinate.
    // - `az`: Axon z-coordinate.
    // - `nt`: Neuron type (0 = Contact, 1 = Sensory, 2 = Motor).
    // - `nrt`: Neurotransmitter type (0 = Inhibitory, 1 = Excitatory).
    pub fn new(x: usize, y: usize, z: usize, ax: usize, ay: usize, az: usize, nt: u32, nrt: u32) -> Self {
        if nt > 2 {
            panic!("error: {} : nt must be 0, 1, or 2", nt);
        }
        if nrt > 1 {
            panic!("error: {} : nrt must be 0 or 1", nrt);
        }

        Neuron {
            x,
            y,
            z,
            ax,
            ay,
            az,
            nt,
            nrt,
            ap: 0.0,
            tp: Self::MIN_THRESHOLD_POTENTIAL,
            mp: Self::RESTING_POTENTIAL,
            fr: 0.0,
            sw: 1.0,
            sst: 0.0,
            pr: 1.0,
            arp: 0.0,
            rrp: Self::BASE_RELATIVE_REFRACTORY_PERIOD,
            ac: HashSet::new(),
            dc: HashSet::new(),
            nc: 1.0,
            ltp: 0.0,
            ltd: 0.0,
        }
    }

    // Establishes the axonal connection with a specified dendritic neuron.
    // Parameters:
    // - `neuron`: A mutable reference to the neuron to connect to.
    pub fn establish_axonal_connection(&mut self, neuron: &mut Neuron) {
        self.ac.insert((neuron.x, neuron.y, neuron.z));
        neuron.dc.insert((self.x, self.y, self.z));
    }
    
    // Establishes the dendritic connection with a specified axonal neuron.
    // Parameters:
    // - `neuron`: A mutable reference to the neuron to connect to.
    pub fn establish_dendritic_connection(&mut self, neuron: &mut Neuron) {
        self.dc.insert((neuron.x, neuron.y, neuron.z));
        neuron.ac.insert((self.x, self.y, self.z));
    }
    
    // Terminates the axonal connection with a specified dendritic neuron.
    // Parameters:
    // - `neuron`: A mutable reference to the neuron to disconnect from.
    pub fn terminate_axonal_connection(&mut self, neuron: &mut Neuron) {
        self.ac.remove(&(neuron.x, neuron.y, neuron.z));
        neuron.dc.remove(&(self.x, self.y, self.z));
    }
    
    // Terminates the dendritic connection with a specified axonal neuron.
    // Parameters:
    // - `neuron`: A mutable reference to the neuron to disconnect from.
    pub fn terminate_dendritic_connection(&mut self, neuron: &mut Neuron) {
        self.dc.remove(&(neuron.x, neuron.y, neuron.z));
        neuron.ac.remove(&(self.x, self.y, self.z));
    }

    // Prunes axonal connections based on the dendritic neuron.
    // Parameters:
    // - `neuron`: A mutable reference to the neuron used for pruning.
    pub fn prune_axonal_connection(&mut self, neuron: &mut Neuron) {
        if self.sw <= self.sst && self.sw < neuron.sw {
            self.terminate_axonal_connection(neuron);
        } else if self.sw >= neuron.sw {
            self.establish_axonal_connection(neuron);
        }
    }

    // Prunes dendritic connections based on the axonal neuron.
    // Parameters:
    // - `neuron`: A mutable reference to the neuron used for pruning.
    pub fn prune_dendritic_connection(&mut self, neuron: &mut Neuron) {
        if self.sw <= self.sst && neuron.sw < self.sw {
            self.terminate_dendritic_connection(neuron);
        } else if neuron.sw >= self.sw {
            self.establish_dendritic_connection(neuron);
        }
    }

    // Detects neuronal signals based on membrane potential.
    // Returns: The generated signal if the membrane potential exceeds the threshold; otherwise, returns 0.0.
    pub fn detect(&mut self) -> f64 {
        if self.mp >= self.tp {
            return self.fire(); // Return the generated signal
        }

        0.0 // No signal triggered, return no signal
    }
    // Transmits signals and accumulates the membrane potential.
    // Parameters:
    // - `input`: The input signal value.
    // - `source`: An optional reference to the source neuron that sends the signal.
    pub async fn transmit(&mut self, input: f64, source: Option<&Neuron>) {
        // Signal delay
        if let Some(neuron) = source {
            let distance = self.calculate_distance(neuron);
            self.signal_delay(distance).await;
        }

        // Check if the neuron is in a refractory state and cannot process incoming signals
        if self.detection_arp() {
            return;
        }

        // Directly use input to accumulate membrane potential
        self.update_ap(input);
        self.update_mp();
        self.update_tp();
        self.update_rp();
        self.update_fr();
        self.update_pr();
        self.update_ltp(input);
        self.update_ltd(input);
        self.update_sst(input);
        self.update_sw();
    }

    // Calculates the distance between this neuron and another neuron.
    // Parameters:
    // - `other`: A reference to the other neuron to calculate distance from.
    // Returns: The Euclidean distance between the two neurons.
    fn calculate_distance(&self, other: &Neuron) -> f64 {
        let xd = (self.x - other.x).pow(2);
        let yd = (self.y - other.y).pow(2);
        let zd = (self.z - other.z).pow(2);
        ((xd + yd + zd) as f64).sqrt() // Return the Euclidean distance
    }
    
    // Delays the signal by a specified unit of time.
    // Parameters:
    // - `unit`: The time unit to delay the signal.
    async fn signal_delay(&self, unit: f64) {
        let millis = unit.round() as u64;
        sleep(Duration::from_millis(millis)).await;
    }

    // Fires the neuron, generating a signal based on its type.
    // Returns: The adjusted signal output based on the neuron's neurotransmitter type.
    fn fire(&mut self) -> f64 {
        let output = match self.nrt {
            1 => (self.ap * (Self::FIRING_RATE_BOOST_FACTOR / self.fr)).clamp(Self::MIN_EXCITATORY_SIGNAL, Self::MAX_EXCITATORY_SIGNAL), // Excitatory signal
            0 => (-self.ap * (Self::FIRING_RATE_BOOST_FACTOR / self.fr)).clamp(Self::MIN_INHIBITORY_SIGNAL, Self::MAX_INHIBITORY_SIGNAL), // Inhibitory signal
            _ => 0.0, // Unknown type
        };
        self.ap = 0.0; // Reset accumulated potential after firing
    
        output // Return the adjusted signal
    }

    // Updates the accumulated potential based on the input signal.
    // Parameters:
    // - `input`: The input signal value to update the accumulated potential.
    fn update_ap(&mut self, input: f64) {
        if input.abs() >= Self::ACCUMULATED_POTENTIAL_CRITICAL_VALUE {
            self.ap += Self::ACCUMULATED_POTENTIAL_STIMULUS_INTENSITY * input * self.nc * self.rrp;
        } else {
            self.ap += Self::ACCUMULATED_POTENTIAL_SLIGHT_INTENSITY * input * self.nc * self.rrp;
        }
    }

    // Updates the membrane potential based on accumulated potential.
    fn update_mp(&mut self) {
        self.mp = (Self::RESTING_POTENTIAL + self.ap).clamp(Self::MIN_MEMBRANE_POTENTIAL, Self::MAX_MEMBRANE_POTENTIAL);
    }

    // Updates the threshold potential based on accumulated potential and firing rate.
    fn update_tp(&mut self) {
        self.tp = Self::MIN_THRESHOLD_POTENTIAL;
        if self.ap > 0.0 {
            self.tp += Self::THRESHOLD_POTENTIAL_BOOST_FACTOR_FOR_ACCUMULATED_POTENTIAL * self.ap;
        }
        self.tp += Self::THRESHOLD_POTENTIAL_BOOST_FACTOR_FOR_FIRING_RATE * self.fr;
        self.tp = self.tp.min(Self::MAX_THRESHOLD_POTENTIAL);
    }

    // Detection of the absolute refractory threshold.
    fn detection_arp(&mut self) -> bool {
        if self.arp > 0.0 {
            self.arp -= Self::ABSOLUTE_REFRACTORY_PERIOD_DECREASE_FACTOR * self.fr;
            self.arp = self.arp.clamp(0.0, Self::BASE_ABSOLUTE_REFRACTORY_PERIOD);
            return true;
        }
        return false;
    }

    // Updates the refractory threshold.
    fn update_rp(&mut self) {
        // Updates the relative refractory threshold.
        if self.rrp < Self::BASE_RELATIVE_REFRACTORY_PERIOD {
            self.rrp += Self::RELATIVE_REFRACTORY_PERIOD_RECOVERY_FACTOR * self.fr;
            self.rrp = self.rrp.min(Self::BASE_RELATIVE_REFRACTORY_PERIOD);
        }
        // Reset refractory threshold
        if self.mp >= self.tp {
            self.arp = Self::BASE_ABSOLUTE_REFRACTORY_PERIOD;
            self.rrp = 0.0;
        }
    }

    // Updates the firing rate based on membrane potential and threshold.
    fn update_fr(&mut self) {
        if self.mp >= self.tp {
            self.fr += Self::FIRING_RATE_BOOST_FACTOR * (self.ap / Self::ACCUMULATED_POTENTIAL_CRITICAL_VALUE);
        } else {
            self.fr *= Self::FIRING_RATE_DECREASE_FACTOR;
        }
        self.fr = self.fr.min(Self::MAX_FIRING_RATE);
    }

    // Updates the plasticity rate based on membrane potential and threshold.
    fn update_pr(&mut self) {
        if self.mp >= self.tp {
            self.pr += Self::PLASTICITY_RATE_BOOST_FACTOR * self.fr;
        } else {
            self.pr *= Self::PLASTICITY_RATE_DECREASE_FACTOR;
        }
        self.pr = self.pr.min(Self::MAX_PLASTICITY_RATE);
    }
    
    // Updates the long-term potentiation based on the input signal.
    // Parameters:
    // - `input`: The input signal value to determine LTP adjustment.
    fn update_ltp(&mut self, input: f64) {
        if input > 0.0 {
            self.ltp += Self::LTP_BOOST_FACTOR * (input / Self::ACCUMULATED_POTENTIAL_CRITICAL_VALUE);
        } else {
            self.ltp *= Self::LTP_DECREASE_FACTOR;
        }
        self.ltp = self.ltp.min(Self::MAX_LTP);
    }
    
    // Updates the long-term depression based on the input signal.
    // Parameters:
    // - `input`: The input signal value to determine LTD adjustment.
    fn update_ltd(&mut self, input: f64) {
        if input < 0.0 {
            self.ltd += Self::LTD_BOOST_FACTOR * (input / Self::ACCUMULATED_POTENTIAL_CRITICAL_VALUE);
        } else {
            self.ltd *= Self::LTD_DECREASE_FACTOR;
        }
        self.ltd = self.ltd.max(Self::MIN_LTD);
    }

    // Updates the synaptic strength threshold based on the input signal.
    // Parameters:
    // - `input`: The input signal value to determine synaptic strength threshold adjustment.
    fn update_sst(&mut self, input: f64) {
        self.sst -= Self::SYNAPTIC_STRENGTH_THRESHOLD_BOOST_FACTOR * (input / Self::ACCUMULATED_POTENTIAL_CRITICAL_VALUE);
        self.sst = self.sst.clamp(Self::MIN_LTD, Self::MAX_LTP);
    }

    // Updates the synaptic weight based on LTD, LTP and plasticity rate.
    fn update_sw(&mut self) {
        self.sw += (self.ltp + self.ltd) * self.pr;
        self.sw = self.sw.clamp(Self::MIN_LTD, Self::MAX_LTP);
    }
}
