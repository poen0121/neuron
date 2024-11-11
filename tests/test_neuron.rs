// tests/test_neuron.rs
use neuron::Neuron;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_creation() {
        let neuron = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);

        assert_eq!(neuron.x, 1, "Expected neuron x position to be 1, got {}", neuron.x);
        assert_eq!(neuron.y, 2, "Expected neuron y position to be 2, got {}", neuron.y);
        assert_eq!(neuron.z, 3, "Expected neuron z position to be 3, got {}", neuron.z);
        assert_eq!(neuron.tp, Neuron::MIN_THRESHOLD_POTENTIAL, "Expected threshold potential to be {}, got {}", Neuron::MIN_THRESHOLD_POTENTIAL, neuron.tp);
    }

    #[test]
    fn test_establish_axonal_connection() {
        let mut neuron1 = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);
        let mut neuron2 = Neuron::new(4, 5, 6, 5, 6, 7, 0, 1);

        neuron1.establish_axonal_connection(&mut neuron2);
        assert!(neuron1.ac.contains(&(4, 5, 6)), "Neuron1 should have an axonal connection to (4, 5, 6)");
        assert!(neuron2.dc.contains(&(1, 2, 3)), "Neuron2 should have a dendritic connection to (1, 2, 3)");
    }

    #[test]
    fn test_terminate_axonal_connection() {
        let mut neuron1 = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);
        let mut neuron2 = Neuron::new(4, 5, 6, 5, 6, 7, 0, 1);
        
        neuron1.establish_axonal_connection(&mut neuron2);
        neuron1.terminate_axonal_connection(&mut neuron2);
        assert!(!neuron1.ac.contains(&(4, 5, 6)), "Neuron1 should not have an axonal connection to (4, 5, 6)");
        assert!(!neuron2.dc.contains(&(1, 2, 3)), "Neuron2 should not have a dendritic connection to (1, 2, 3)");
    }

    #[test]
    fn test_establish_dendritic_connection() {
        let mut neuron1 = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);
        let mut neuron2 = Neuron::new(4, 5, 6, 5, 6, 7, 0, 1);

        neuron1.establish_dendritic_connection(&mut neuron2);
        assert!(neuron1.dc.contains(&(4, 5, 6)), "Neuron1 should have a dendritic connection to (4, 5, 6)");
        assert!(neuron2.ac.contains(&(1, 2, 3)), "Neuron2 should have an axonal connection to (1, 2, 3)");
    }

    #[test]
    fn test_terminate_dendritic_connection() {
        let mut neuron1 = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);
        let mut neuron2 = Neuron::new(4, 5, 6, 5, 6, 7, 0, 1);
        
        neuron1.establish_dendritic_connection(&mut neuron2);
        neuron1.terminate_dendritic_connection(&mut neuron2);
        assert!(!neuron1.dc.contains(&(4, 5, 6)), "Neuron1 should not have a dendritic connection to (4, 5, 6)");
        assert!(!neuron2.ac.contains(&(1, 2, 3)), "Neuron2 should not have an axonal connection to (1, 2, 3)");
    }

    #[test]
    fn test_prune_axonal_connection() {
        let mut neuron1 = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);
        let mut neuron2 = Neuron::new(4, 5, 6, 5, 6, 7, 0, 1);

        neuron1.sw = 0.05; 
        neuron1.sst = 0.05; 
        neuron2.sw = 0.1; 
        neuron1.establish_axonal_connection(&mut neuron2);
        neuron1.prune_axonal_connection(&mut neuron2);

        assert!(!neuron1.ac.contains(&(4, 5, 6)), "({}, {}, {}) -> Neuron1 should not have an axonal connection to (4, 5, 6) after pruning", neuron1.x, neuron1.y, neuron1.z);
        assert!(!neuron2.dc.contains(&(1, 2, 3)), "({}, {}, {}) -> Neuron2 should not have a dendritic connection to (1, 2, 3) after pruning", neuron2.x, neuron2.y, neuron2.z);
    }

    #[test]
    fn test_prune_dendritic_connection() {
        let mut neuron1 = Neuron::new(1, 1, 1, 2, 2, 2, 0, 1);
        let mut neuron2 = Neuron::new(1, 2, 3, 2, 3, 4, 0, 1);

        neuron1.sw = 0.05; 
        neuron2.sw = 0.1; 
        neuron2.sst = 0.1; 
        neuron1.establish_axonal_connection(&mut neuron2); 
        neuron2.prune_dendritic_connection(&mut neuron1);
        
        assert!(!neuron2.dc.contains(&(1, 1, 1)), "({}, {}, {}) -> Neuron2 should not have a dendritic connection to (1, 1, 1) after pruning", neuron2.x, neuron2.y, neuron2.z);
        assert!(!neuron1.ac.contains(&(1, 2, 3)), "({}, {}, {}) -> Neuron1 should not have an axonal connection to (1, 2, 3) after pruning", neuron1.x, neuron1.y, neuron1.z);
    }

    #[tokio::test]
    async fn test_signal_accumulation_and_firing() {
        let mut neuron0 = Neuron::new(0, 0, 0, 1, 1, 1, 1, 0);
        let mut neuron1 = Neuron::new(1, 1, 1, 2, 2, 2, 1, 1);
        let mut neuron2 = Neuron::new(1, 2, 3, 2, 3, 4, 2, 1);

        neuron0.transmit(20.0, None).await;
        let output = neuron0.detect();
        assert!(output < 0.0, "({}, {}, {}) -> Accumulated potential: {} -> Signal output: {}", neuron0.x, neuron0.y, neuron0.z, neuron0.ap, output);
    
        neuron2.transmit(output, Some(&mut neuron0)).await;
        let output = neuron2.detect();
        assert_eq!(output, 0.0, "({}, {}, {}) -> Expected output from neuron2 to be 0.0 after signaling from neuron0, got {}", neuron2.x, neuron2.y, neuron2.z, output);

        loop {
            neuron1.transmit(20.0, None).await;
            let mut output = neuron1.detect();
            neuron2.transmit(output, Some(&mut neuron1)).await;
            output = neuron2.detect();
            if output > 0.0 {
                assert!(output > 0.0, "({}, {}, {}) -> Expected output from neuron2 to be greater than 0 after signaling from neuron1, got {}", neuron2.x, neuron2.y, neuron2.z, output);
                assert_eq!(neuron2.ap, 0.0, "({}, {}, {}) -> Expected neuron2 accumulated potential to be 0.0 after signaling, got {}", neuron2.x, neuron2.y, neuron2.z, neuron2.ap);
                break;
            }
        }
    }

    #[tokio::test]
    async fn test_synaptic_weight_changes() {
        let mut neuron1 = Neuron::new(1, 1, 1, 2, 2, 2, 1, 1);
        let mut neuron2 = Neuron::new(1, 2, 3, 2, 3, 4, 1, 1);

        neuron1.transmit(20.0, None).await;
        
        assert!(neuron1.mp > neuron1.tp, "({}, {}, {}) -> Expected neuron1 mp >= tp , got mp{}, tp{}", neuron1.x, neuron1.y, neuron1.z, neuron1.mp, neuron1.tp);
        assert!(neuron1.sw > -1.0, "({}, {}, {}) -> Expected neuron1 synaptic weight to increase after signaling, got {}", neuron1.x, neuron1.y, neuron1.z, neuron1.sw);

        neuron2.transmit(-20.0, None).await;
        assert!(neuron2.mp < neuron2.tp, "({}, {}, {}) -> Expected neuron2 mp < tp , got mp{}, tp{}", neuron1.x, neuron1.y, neuron1.z, neuron1.mp, neuron1.tp);
        assert!(neuron2.sw < 1.0, "({}, {}, {}) -> Expected neuron2 synaptic weight to decrease after signaling with reduced weight, got {}", neuron2.x, neuron2.y, neuron2.z, neuron1.sw);
    }
}
