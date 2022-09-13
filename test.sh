scp /Users/brycebartmann/Atlo-Protocol/repos/vesting_binding_test/artifacts/vestingtest.wasm root@116.203.146.73:

cored tx wasm store vestingtest.wasm --from kujira16fhxs9ucl45tnansmrr62rrf93quq99t2f35lm --keyring-backend test --gas 20000000 --chain-id "harpoon-2"

cored tx wasm instantiate "13" "{}" --label test_vest --admin kujira16fhxs9ucl45tnansmrr62rrf93quq99t2f35lm --from kujira16fhxs9ucl45tnansmrr62rrf93quq99t2f35lm --keyring-backend test --gas 20000000 --chain-id "harpoon-2"

cored tx bank send kujira16fhxs9ucl45tnansmrr62rrf93quq99t2f35lm kujira1zwv6feuzhy6a9wekh96cd57lsarmqlwxdypdsplw6zhfncqw6ftqt9g73r 20000000ukuji --from kujira16fhxs9ucl45tnansmrr62rrf93quq99t2f35lm --keyring-backend test --gas 20000000 --chain-id "harpoon-2"

cored tx wasm execute kujira1l6z44fupg8l5mapmuxnrnmvxyclw5wnh5f5hfrtkaq5r0d9s24mss4s0qk '{"tokens_transfer_vest": {"to_address": "kujira19qsjyg5qmveqjttdnfm2al5yr7g30e0d65luda", "vest_end_time": "1663682346000000000" }}' --amount 20000000ukuji --from kujira16fhxs9ucl45tnansmrr62rrf93quq99t2f35lm --keyring-backend test --gas 20000000 --chain-id "harpoon-2"