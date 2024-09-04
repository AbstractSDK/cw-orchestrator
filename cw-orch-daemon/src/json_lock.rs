use crate::DaemonError;
use file_lock::{FileLock, FileOptions};
use serde_json::{from_reader, json, Value};
use std::{fs::File, io::Seek};

/// State file reader and writer
/// Mainly used by [`crate::Daemon`] and [`crate::DaemonAsync`], but could also be used for tests or custom edits of the state
#[derive(Debug)]
pub struct JsonLockedState {
    lock: FileLock,
    json: Value,
    path: String,
}

impl JsonLockedState {
    /// Lock a state files
    /// Other process won't be able to lock it
    pub fn new(path: &str) -> Self {
        // open file pointer set read/write permissions to true
        // create it if it does not exists
        // don't truncate it

        let options = FileOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false);

        // Lock file, non blocking so it errors in case someone else already holding lock of it
        let lock: FileLock = FileLock::lock(path, false, options)
            .unwrap_or_else(|_| panic!("Was not able to receive {path} state lock"));

        // return empty json object if file is empty
        // return file content if not
        let json: Value = if lock.file.metadata().unwrap().len().eq(&0) {
            json!({})
        } else {
            let json: Value = from_reader(&lock.file).unwrap();
            patch_state_if_old(json)
        };

        let filename = path.to_owned();

        JsonLockedState {
            lock,
            json,
            path: filename,
        }
    }

    /// Prepare json for further writes
    pub fn prepare(&mut self, chain_id: &str, deploy_id: &str) {
        let json = &mut self.json;

        // add deployment_id to chain_id path
        if json.get(chain_id).is_none() {
            json[chain_id] = json!({
                deploy_id: {},
                "code_ids": {}
            });
        }
    }

    pub fn state(&self) -> Value {
        self.json.clone()
    }

    /// Get a value for read
    pub fn get(&self, chain_id: &str) -> &Value {
        &self.json[chain_id]
    }

    /// Give a value to write
    pub fn get_mut(&mut self, chain_id: &str) -> &mut Value {
        self.json.get_mut(chain_id).unwrap()
    }

    /// Force write to a file
    pub fn force_write(&mut self) {
        self.lock.file.set_len(0).unwrap();
        self.lock.file.rewind().unwrap();
        serde_json::to_writer_pretty(&self.lock.file, &self.json).unwrap();
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

// Write json when dropping
impl Drop for JsonLockedState {
    fn drop(&mut self) {
        self.force_write()
    }
}

pub fn read(filename: &String) -> Result<Value, DaemonError> {
    let file = File::open(filename)
        .map_err(|err| DaemonError::OpenFile(filename.to_string(), err.to_string()))?;
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}

pub(crate) fn patch_state_if_old(maybe_old: Value) -> Value {
    let expect_object = |v: Value| -> serde_json::Map<String, Value> {
        let Value::Object(map) = v else {
            panic!("Unexpected daemon state format");
        };
        map
    };

    let maybe_old_map = expect_object(maybe_old);
    let mut maybe_old_iter = maybe_old_map.iter();
    let Some((_maybe_chain_name, maybe_chain_id_object)) = maybe_old_iter.next() else {
        // Empty map
        return Value::Object(maybe_old_map);
    };
    let Value::Object(maybe_chain_map) = maybe_chain_id_object else {
        panic!("Unexpected daemon state format");
    };
    if maybe_chain_map.iter().any(|(key, _val)| key == "code_ids") {
        // It's new format we good
        return Value::Object(maybe_old_map);
    }
    // Assuming it's an old daemon state from now on as we didn't found code id object under first key
    // We just need to join all chain maps
    let new_state = maybe_old_map.into_iter().fold(
        serde_json::Map::new(),
        |mut acc, (_chain_name, chain_value)| {
            let chain_map = expect_object(chain_value);
            acc.extend(chain_map);
            acc
        },
    );
    Value::Object(new_state)
}

#[cfg(test)]
mod test_old_patch {
    use super::*;

    #[test]
    fn old_to_new() {
        let old_map = json!({
            "chain-name": {
                "chain-id": {
                    "abracadabra": {
                        "open": "sesame"
                    },
                    "code_ids": {
                        "foo": 123
                    }
                }
            }
        });
        let patched = patch_state_if_old(old_map);
        let expected = json!({
            "chain-id": {
                    "abracadabra": {
                        "open": "sesame"
                    },
                    "code_ids": {
                        "foo": 123
                    }
                }
        });
        assert_eq!(patched, expected);
        // Already new map, nothing to patch
        let not_patched = patch_state_if_old(patched);
        assert_eq!(not_patched, expected);
    }

    #[test]
    fn big_test() {
        let old_starship_state = json!({
                "juno": {
                  "juno-1": {
                    "code_ids": {
                      "abstract:account-factory": 22,
                      "abstract:ans-host": 20,
                      "abstract:ibc-client": 26,
                      "abstract:ibc-host": 27,
                      "abstract:manager": 24,
                      "abstract:module-factory": 23,
                      "abstract:proxy": 25,
                      "abstract:version-control": 21,
                      "polytone:note": 9,
                      "polytone:proxy": 11,
                      "polytone:voice": 10
                    },
                    "default": {
                      "abstract:account-factory": "juno1j6scewlz8mrsk3dl3a578rmdps8svlz8jltwpa5ee2qsj8va8ats2ksmgn",
                      "abstract:ans-host": "juno1vmsr7jrfsaveqeqpua2cpfz6wtsrh7ym2wlk82ghpltjqcg9psrs76yzld",
                      "abstract:ibc-client": "juno13nh52udymffggkjaq4vqa8ds0hm4uv0eedzvyq43szpwkwjqu07qdhknsf",
                      "abstract:ibc-host": "juno1cznnmdqxg86ysplcese8z5t504gyhxzndhfuktns6chejk0wvh8sfttefv",
                      "abstract:manager-local-0": "juno1xthk3jr7ful9xmxqqsup7pkx9ngkycq7hfa7t75gcv47ajpyjvgsdxtlqs",
                      "abstract:manager-local-1": "juno1ha8kxcqcclt8a03fryt0xrpps2jzhyfdv6aaltu5e6wn7l4v0rjs9khssu",
                      "abstract:manager-local-2": "juno1k8e4pg220xyy8xltptxvt4w2rkajjfr05u0007c5j0ad2ntxmslqhlntp0",
                      "abstract:module-factory": "juno19gqaw2laku723sz32aq75pc6rw9q9fy9ruq6l0jqg272utkp4jdsmev49q",
                      "abstract:proxy-local-0": "juno1a0gu37sj27hpndjjsse9mdx3n5j46qz8mwqghwtsv98gztumr8jsa3u7w3",
                      "abstract:proxy-local-1": "juno1dwsvlcpjvdwyrka0smftpu8mln4q8dadv2xz3xxl92ngfxvuf5ss82crwn",
                      "abstract:proxy-local-2": "juno17x2jjjafl4mlqvtd22tv2sptjlw3hnwgxj85wqasfd0temhxzpfsgn8guv",
                      "abstract:version-control": "juno10u2skqwln29qrw2ggg5xhkw8t0xct6jz4674urcdx5qj0uyl08eq6jq0qa",
                      "polytone:note": null,
                      "polytone:note | junotwo-1": "juno1plr28ztj64a47a32lw7tdae8vluzm2lm7nqk364r4ws50rgwyzgs6y8ewu"
                    }
                  },
                  "junofour-1": {
                    "code_ids": {},
                    "default": {}
                  },
                  "junothree-1": {
                    "code_ids": {},
                    "default": {}
                  }
                },
                "osmosis": {
                  "junotwo-1": {
                    "code_ids": {
                      "abstract:account-factory": 24,
                      "abstract:ans-host": 22,
                      "abstract:ibc-client": 28,
                      "abstract:ibc-host": 29,
                      "abstract:manager": 26,
                      "abstract:module-factory": 25,
                      "abstract:proxy": 27,
                      "abstract:version-control": 23,
                      "counter_contract": 30,
                      "polytone:note": 9,
                      "polytone:proxy": 11,
                      "polytone:voice": 10
                    },
                    "default": {
                      "abstract:account-factory": "osmo1nv772ju2szazve72a5nycsscam770gq5ranufcmsj9n8mkm86yjqwvm4ft",
                      "abstract:ans-host": "osmo1ud7zhcceh4hwjhuhs58nxnh9he0n7sgx90wqq7uslr67k8vuj2pq6fynqv",
                      "abstract:ibc-client": "osmo1sun3clwczfm2p0xtyhesnpchdqcj55d2wzxjjgd0hqtxejczdypshlwrzy",
                      "abstract:ibc-host": "osmo149fzg5j97794hu3f09w8eh54v26xnwym8s0decq6ll9zfgn80g5smpypkg",
                      "abstract:manager-juno-2": "osmo1ng8jq8a0chlx73shyu998ytvlz5384h45u8xh6gc5gh63p2sv3aqdzhtep",
                      "abstract:manager-local-0": "osmo1wf0rljs9zselxp3kyg6vwtsg5k9nd2c5s38kuzde8vgzk46r9qssgtkjdy",
                      "abstract:module-factory": "osmo1ekc95e6p7277t77ahxn2dhl5qz76r6egdlrdp2ehvewdraa97m7qmyt5vn",
                      "abstract:proxy-juno-2": "osmo1d2h356rtl4yex2cnl7au7nrq262mvxqvtclde3lrzqg0kesrcrhqah9xgp",
                      "abstract:proxy-local-0": "osmo1nncn9p7xafk7zfnt4c4jy8mqh3m25t2tfw0scsghvmc485uxnzxqrtpd30",
                      "abstract:version-control": "osmo1wqchrjh07e3kxaee59yrpzckwr94j03zchmdslypvkv6ps0684msrzv484",
                      "counter_contract": "osmo1t4a34yj7r7h7fffuls3gvj6ept6vqyuujm4v2a4lnrnm8yckwmuqnkd0a9",
                      "polytone:voice": null,
                      "polytone:voice | juno-1": "osmo1hzz0s0ucrhdp6tue2lxk3c03nj6f60qy463we7lgx0wudd72ctms64096d"
                    }
                  }
                }
              }
        );
        let expected = json!({
            "juno-1": {
              "code_ids": {
                "abstract:account-factory": 22,
                "abstract:ans-host": 20,
                "abstract:ibc-client": 26,
                "abstract:ibc-host": 27,
                "abstract:manager": 24,
                "abstract:module-factory": 23,
                "abstract:proxy": 25,
                "abstract:version-control": 21,
                "polytone:note": 9,
                "polytone:proxy": 11,
                "polytone:voice": 10
              },
              "default": {
                "abstract:account-factory": "juno1j6scewlz8mrsk3dl3a578rmdps8svlz8jltwpa5ee2qsj8va8ats2ksmgn",
                "abstract:ans-host": "juno1vmsr7jrfsaveqeqpua2cpfz6wtsrh7ym2wlk82ghpltjqcg9psrs76yzld",
                "abstract:ibc-client": "juno13nh52udymffggkjaq4vqa8ds0hm4uv0eedzvyq43szpwkwjqu07qdhknsf",
                "abstract:ibc-host": "juno1cznnmdqxg86ysplcese8z5t504gyhxzndhfuktns6chejk0wvh8sfttefv",
                "abstract:manager-local-0": "juno1xthk3jr7ful9xmxqqsup7pkx9ngkycq7hfa7t75gcv47ajpyjvgsdxtlqs",
                "abstract:manager-local-1": "juno1ha8kxcqcclt8a03fryt0xrpps2jzhyfdv6aaltu5e6wn7l4v0rjs9khssu",
                "abstract:manager-local-2": "juno1k8e4pg220xyy8xltptxvt4w2rkajjfr05u0007c5j0ad2ntxmslqhlntp0",
                "abstract:module-factory": "juno19gqaw2laku723sz32aq75pc6rw9q9fy9ruq6l0jqg272utkp4jdsmev49q",
                "abstract:proxy-local-0": "juno1a0gu37sj27hpndjjsse9mdx3n5j46qz8mwqghwtsv98gztumr8jsa3u7w3",
                "abstract:proxy-local-1": "juno1dwsvlcpjvdwyrka0smftpu8mln4q8dadv2xz3xxl92ngfxvuf5ss82crwn",
                "abstract:proxy-local-2": "juno17x2jjjafl4mlqvtd22tv2sptjlw3hnwgxj85wqasfd0temhxzpfsgn8guv",
                "abstract:version-control": "juno10u2skqwln29qrw2ggg5xhkw8t0xct6jz4674urcdx5qj0uyl08eq6jq0qa",
                "polytone:note": null,
                "polytone:note | junotwo-1": "juno1plr28ztj64a47a32lw7tdae8vluzm2lm7nqk364r4ws50rgwyzgs6y8ewu"
              }
            },
            "junofour-1": {
              "code_ids": {},
              "default": {}
            },
            "junothree-1": {
              "code_ids": {},
              "default": {}
            },
            "junotwo-1": {
              "code_ids": {
                "abstract:account-factory": 24,
                "abstract:ans-host": 22,
                "abstract:ibc-client": 28,
                "abstract:ibc-host": 29,
                "abstract:manager": 26,
                "abstract:module-factory": 25,
                "abstract:proxy": 27,
                "abstract:version-control": 23,
                "counter_contract": 30,
                "polytone:note": 9,
                "polytone:proxy": 11,
                "polytone:voice": 10
              },
              "default": {
                "abstract:account-factory": "osmo1nv772ju2szazve72a5nycsscam770gq5ranufcmsj9n8mkm86yjqwvm4ft",
                "abstract:ans-host": "osmo1ud7zhcceh4hwjhuhs58nxnh9he0n7sgx90wqq7uslr67k8vuj2pq6fynqv",
                "abstract:ibc-client": "osmo1sun3clwczfm2p0xtyhesnpchdqcj55d2wzxjjgd0hqtxejczdypshlwrzy",
                "abstract:ibc-host": "osmo149fzg5j97794hu3f09w8eh54v26xnwym8s0decq6ll9zfgn80g5smpypkg",
                "abstract:manager-juno-2": "osmo1ng8jq8a0chlx73shyu998ytvlz5384h45u8xh6gc5gh63p2sv3aqdzhtep",
                "abstract:manager-local-0": "osmo1wf0rljs9zselxp3kyg6vwtsg5k9nd2c5s38kuzde8vgzk46r9qssgtkjdy",
                "abstract:module-factory": "osmo1ekc95e6p7277t77ahxn2dhl5qz76r6egdlrdp2ehvewdraa97m7qmyt5vn",
                "abstract:proxy-juno-2": "osmo1d2h356rtl4yex2cnl7au7nrq262mvxqvtclde3lrzqg0kesrcrhqah9xgp",
                "abstract:proxy-local-0": "osmo1nncn9p7xafk7zfnt4c4jy8mqh3m25t2tfw0scsghvmc485uxnzxqrtpd30",
                "abstract:version-control": "osmo1wqchrjh07e3kxaee59yrpzckwr94j03zchmdslypvkv6ps0684msrzv484",
                "counter_contract": "osmo1t4a34yj7r7h7fffuls3gvj6ept6vqyuujm4v2a4lnrnm8yckwmuqnkd0a9",
                "polytone:voice": null,
                "polytone:voice | juno-1": "osmo1hzz0s0ucrhdp6tue2lxk3c03nj6f60qy463we7lgx0wudd72ctms64096d"
              }
            }
        });
        let patched = patch_state_if_old(old_starship_state);
        assert_eq!(patched, expected);
        // Already new map, nothing to patch
        let not_patched = patch_state_if_old(patched);
        assert_eq!(not_patched, expected);
    }
}
