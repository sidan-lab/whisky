#[cfg(test)]
mod tx_parser {
    use whisky_js::js_get_tx_outs_utxo;

    #[test]
    fn test_parse_tx_body() {
        let tx_hex = "84a300d90102818258202cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd8503018282581d60f95cab9352c14782a366802b7967746a89356e8915c17006149ff68c0082581d60f95cab9352c14782a366802b7967746a89356e8915c17006149ff68c1b000000024d95f5570200a0f5f6";
        let result = js_get_tx_outs_utxo(tx_hex);
        println!("{:?}", result);
    }
}
