# Neuron Simulation

## About

This code aims to simulate the behavior of neurons, including the reception and transmission of signals, the formation and pruning of synaptic connections, and the consideration of neural plasticity. 

The simulation is implemented using the Rust programming language.

## Neuron Structure

The code defines a structure named `Neuron`.

Below are the main components explained in detail:

- **`x`, `y`, `z`**: 3D coordinates of the neuron.
- **`ax`, `ay`, `az`**: 3D coordinates of the neuron axon.
- **`nt`**: Type of neuron (0 = contact neuron, 1 = sensory neuron, 2 = motor neuron).
- **`nrt`**: Type of neurotransmitter (0 = inhibitory, 1 = excitatory).
- **`ap`**: Accumulated potential.
- **`tp`**: Threshold potential.
- **`mp`**: Membrane potential.
- **`fr`**: Firing rate of the neuron.
- **`sw`**: Synaptic weight.
- **`sst`**: Synaptic strength threshold.
- **`pr`**: Plasticity rate.
- **`arp`**: Absolute refractory period.
- **`rrp`**: Relative refractory period.
- **`ac`**: Axonal connections.
- **`dc`**: Dendritic connections.
- **`nc`**: Concentration of neurotransmitters.
- **`ltp`, `ltd`**: Long-term potentiation and depression factors.

## Neuron Functionality

This code defines the functionality of the `Neuron` structure.

Below are the main components explained in detail:

- **`new`**: Create a new instance of a neuron.
- **`establish_axonal_connection`**: Establish an axonal connection with another neuron.
- **`establish_dendritic_connection`**: Establish a dendritic connection with another neuron.
- **`terminate_axonal_connection`**: Terminate an axonal connection with another neuron.
- **`terminate_dendritic_connection`**: Terminate a dendritic connection with another neuron.
- **`prune_axonal_connection`**: Prune axonal connections based on synaptic strength.
- **`prune_dendritic_connection`**: Prune dendritic connections based on synaptic strength.
- **`detect`**: Detect signals from the neuron; fires if membrane potential exceeds the threshold.
- **`transmit`**: Transmits signals and update membrane potential.

## License

This project is licensed under the GNU General Public License (GPL) v3.0. You can freely use, modify, and distribute the code, but any derivative works must also be licensed under the GPL, and the source code must be made available.

See the full GPL license text at https://www.gnu.org/licenses/gpl-3.0.html.