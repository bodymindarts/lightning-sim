use secp256k1::PublicKey;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct Node {
    pub pub_key: PublicKey,
}

#[derive(Debug, Deserialize)]
struct Edge {
    pub node1_pub: PublicKey,
    pub node2_pub: PublicKey,
    pub capacity: u64,
}

#[cfg(test)]
mod test {
    use super::*;
    use stringreader::*;

    #[test]
    fn deserialize_node() {
        let node_str = r#"{
  "pub_key": "020003b9499a97c8dfbbab6b196319db37ba9c37bccb60477f3c867175f417988e"
}"#;

        let node_reader = StringReader::new(node_str);
        let node: Node = serde_json::from_reader(node_reader).unwrap();

        assert_eq!(
            node.pub_key,
            PublicKey::from_str(
                "020003b9499a97c8dfbbab6b196319db37ba9c37bccb60477f3c867175f417988e"
            )
            .unwrap()
        );
    }
}
