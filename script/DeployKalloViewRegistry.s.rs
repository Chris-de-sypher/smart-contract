// DeployKalloViewRegistry.s.rs (Converted from Solidity)
use forge_std::Script;
use crate::src::KalloViewRegistry;

contract! {
    DeployKalloViewRegistry {
        fn set_up(&mut self) {
        }

        fn run(&mut self) {
            let deployer_private_key = vm.env_uint("PK");
            vm.start_broadcast(deployer_private_key);

            let kallo = KalloViewRegistry::new();
        }
    }
}