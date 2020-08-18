pub mod user_contents;
pub mod include;
pub mod file;
pub mod content;
mod util;
mod zeruformatter;

pub use user_contents::UserContents;
pub use include::Include;
pub use file::File;
pub use content::Content;

#[cfg(test)]
#[cfg_attr(tarpaulin, ignore)]
mod tests {
	use super::*;

	#[test]
	fn test_verification() {
		let content: Content = serde_json::from_str(r#"
		{
			"address": "1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52",
			"address_index": 36579623,
			"background-color": "white",
			"cloneable": true,
			"cloned_from": "1RedkCkVaXuVXrqCMpoXQS29bwaqsuFdL",
			"description": "Home of the bots",
			"files": {
			 "data-default/users/content.json-default": {
				"sha512": "4e37699bd5336b9c33ce86a3eb73b82e87460535793401874a653afeddefee59",
				"size": 735
			 },
			 "index.html": {
				"sha512": "087c6ae46aacc5661f7da99ce10dacc0428dbd48aa7bbdc1df9c2da6e81b1d93",
				"size": 466
			 }
			},
			"ignore": "((js|css)/(?!all.(js|css))|data/.*db|data/users/.*/.*)",
			"includes": {
			 "data/users/content.json": {
				"signers": [],
				"signers_required": 1
			 }
			},
			"inner_path": "content.json",
			"merged_type": "ZeroMe",
			"modified": 1471656205.079839,
			"postmessage_nonce_security": true,
			"sign": [
			 60601328857260736769667767617236149396007806053808183569130735997086722937268,
			 43661716327244911082383801335054839207111588960552431293232589470692186442781
			],
			"signers_sign": "HEMH4/a7LXic4PYgMj/4toV5jI5z+SX6Bnmo3mP0HoyIGy6e7rUbilJYAH3MrgCT/IXzIn7cnIlhL8VARh7CeUg=",
			"signs": {
			 "1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52": "G5qMkd9+n0FMLm2KA4FAN3cz/vaGY/oSYd2k/edx4C+TIv76NQI37NsjXVWtkckMoxvp6rhW8PHZy9Q1MNtmIAM="
			},
			"signs_required": 1,
			"title": "Bot Hub",
			"zeronet_version": "0.4.0"
		 }"#).unwrap();
		let key = String::from("1JUDmCT4UCSdnPsJAHBoXNkDS61Y31Ue52");
		let result = content.verify(key);
		assert_eq!(result, true)
	}
}
