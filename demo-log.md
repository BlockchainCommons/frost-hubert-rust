## Set zsh options

zsh is the default shell on macOS and many Linux systems. This keeps history markers out of the transcript.

```
setopt nobanghist
```

## Checking prerequisites

Verify that the required CLI tools are present and available in $PATH.

```
for cmd in frost envelope; do
  $cmd --version
done

│ frost 0.1.0
│ bc-envelope-cli 0.27.0
```

## Preparing demo workspace

Start with a clean directory to capture demo artifacts.

```
rm -rf demo && mkdir -p demo
```

## Provisioning XID for Alice

Generate Alice's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
ALICE_PRVKEYS=$(envelope generate prvkeys)
echo "ALICE_PRVKEYS=$ALICE_PRVKEYS"
ALICE_PUBKEYS=$(envelope generate pubkeys "$ALICE_PRVKEYS")
echo "ALICE_PUBKEYS=$ALICE_PUBKEYS"
ALICE_OWNER_DOC=$(envelope xid new --nickname Alice --sign inception "$ALICE_PRVKEYS")
echo "ALICE_OWNER_DOC=$ALICE_OWNER_DOC"
ALICE_SIGNED_DOC=$(envelope xid new --nickname Alice --private omit --sign inception "$ALICE_PRVKEYS")
echo "ALICE_SIGNED_DOC=$ALICE_SIGNED_DOC"

│ ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxgwhpoxlfaeltuydpbsidlkgsclhfenzmwfprfdswskgmjtpalnoskiflfldwctfhtansgehdcxtefnbnqdvldevlbwneemkkcezslkmsksftksprtatknsjenlwlmnjnmkwfkbhfhywepyfxsg
│ ALICE_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwdlbzbwpf
│ ALICE_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswoyaylrtpsotansgylftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxgwhpoxlfaeltuydpbsidlkgsclhfenzmwfprfdswskgmjtpalnoskiflfldwctfhtansgehdcxtefnbnqdvldevlbwneemkkcezslkmsksftksprtatknsjenlwlmnjnmkwfkbhfhyoybstpsotansgmhdcxiozsahtovozegyiejnmemhseihktflempdjnloldrndiecwztohpnbemmoasiamuoyaxtpsotansghhdfzpswmetlaadkplpcskshtlstsamwnttsgspbkzsfyykfygapscfctctrsgojoaaqznelswyrksgdpkecklfmwlrplbbfskocawkdeylpaetkkzsrhwsqzmofspsdydlmnnnpmrklt
│ ALICE_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswoyaylstpsotansgylftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzzsinuyleaacplkrehkswynwseydnksbsfnfsvsgehgweiepybzbdbndsbacedebztstlnsvdrkhgbkgwwnididlpaebsjpluhhrokbssialtiepeurkoadlomwwfchtksbryfhhn
```

## Provisioning XID for Bob

Generate Bob's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
BOB_PRVKEYS=$(envelope generate prvkeys)
echo "BOB_PRVKEYS=$BOB_PRVKEYS"
BOB_PUBKEYS=$(envelope generate pubkeys "$BOB_PRVKEYS")
echo "BOB_PUBKEYS=$BOB_PUBKEYS"
BOB_OWNER_DOC=$(envelope xid new --nickname Bob --sign inception "$BOB_PRVKEYS")
echo "BOB_OWNER_DOC=$BOB_OWNER_DOC"
BOB_SIGNED_DOC=$(envelope xid new --nickname Bob --private omit --sign inception "$BOB_PRVKEYS")
echo "BOB_SIGNED_DOC=$BOB_SIGNED_DOC"

│ BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxatckeheeoetewlatlruttdnetiprcygapfdauortknwyknmegewnvslofxkgehestansgehdcxdkhtwkasidtitijssptiytlagthpfhvasstkkiptnyhsghkbnbreaoryhpoylblyltghpely
│ BOB_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpyksnttbk
│ BOB_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonoyaylrtpsotansgylftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpoycsfncsfglfoycsfptpsotansgtlftansgohdcxatckeheeoetewlatlruttdnetiprcygapfdauortknwyknmegewnvslofxkgehestansgehdcxdkhtwkasidtitijssptiytlagthpfhvasstkkiptnyhsghkbnbreaoryhpoylblyoybstpsotansgmhdcxndwnfwdskejshgmkpyeeetwnwpspzmtarftirtwtemfwihetjkdmfrhpueqdflwyoycscstpsoiafwjlidoyaxtpsotansghhdfzheeynskepsadsetsdsjtcpsswndwprrekkwectvlmtbzbeoeottecknbuypetyurhhpefyztlkbyclrsfeaybkteuotyhlfsgsemfhtibepttyflidvtswfnhsmnwmghlfhyieto
│ BOB_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonoyaylstpsotansgylftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzahatnewtwegurhlgwmtyrhprvdfyjeotgabzqztpqdsfinnbmdishhlezekiwelgropdolspntaaatjprlrllddmrtosjpihjyveesrhrlwfjlrogrrkwlwtgofptshegoetsbds
```

## Provisioning XID for Carol

Generate Carol's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
CAROL_PRVKEYS=$(envelope generate prvkeys)
echo "CAROL_PRVKEYS=$CAROL_PRVKEYS"
CAROL_PUBKEYS=$(envelope generate pubkeys "$CAROL_PRVKEYS")
echo "CAROL_PUBKEYS=$CAROL_PUBKEYS"
CAROL_OWNER_DOC=$(envelope xid new --nickname Carol --sign inception "$CAROL_PRVKEYS")
echo "CAROL_OWNER_DOC=$CAROL_OWNER_DOC"
CAROL_SIGNED_DOC=$(envelope xid new --nickname Carol --private omit --sign inception "$CAROL_PRVKEYS")
echo "CAROL_SIGNED_DOC=$CAROL_SIGNED_DOC"

│ CAROL_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxqdeeneqdzebyskeelnmngemdrnmhwkwsldkkmwnsgdhkgecszoytfxprlaaogysrtansgehdcxwypmpdwtledetbemahgtveeopmlklussnebbdwiacsdwnldtamwfgyhdpesekiahmoetfpdr
│ CAROL_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtsgwtlymk
│ CAROL_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgoyaylrtpsotansgylftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtoycsfncsfglfoycsfptpsotansgtlftansgohdcxqdeeneqdzebyskeelnmngemdrnmhwkwsldkkmwnsgdhkgecszoytfxprlaaogysrtansgehdcxwypmpdwtledetbemahgtveeopmlklussnebbdwiacsdwnldtamwfgyhdpesekiahoybstpsotansgmhdcxotaacpghidtslemnjopdgrykcmwpveattlltsbnbvafdgeqdmwqdwndmspbwpetkoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzadlsldksfmjpspeetodtfmpfykgdlnahfndldeiswmnehkfyptftcxylrneofrsfndgrnyckmejzeoaeamsndiskehssdehhsodkpeonaadlckykrnjorokecabwbwbbmuasgdki
│ CAROL_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgoyaylstpsotansgylftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzylwpmtwpsbhphlkgghgtftpyfxdpptgylslrnyparojyclpanntymhoyctwyvlnyuomscmjzkkoncerpclaxzseyecmhrpykytrhmejspystiyhholldsbskdiectljopfaeuyot
```

## Provisioning XID for Dan

Generate Dan's key material, a private XID document (for owner use), and a signed public XID document (for participants).

```
DAN_PRVKEYS=$(envelope generate prvkeys)
echo "DAN_PRVKEYS=$DAN_PRVKEYS"
DAN_PUBKEYS=$(envelope generate pubkeys "$DAN_PRVKEYS")
echo "DAN_PUBKEYS=$DAN_PUBKEYS"
DAN_OWNER_DOC=$(envelope xid new --nickname Dan --sign inception "$DAN_PRVKEYS")
echo "DAN_OWNER_DOC=$DAN_OWNER_DOC"
DAN_SIGNED_DOC=$(envelope xid new --nickname Dan --private omit --sign inception "$DAN_PRVKEYS")
echo "DAN_SIGNED_DOC=$DAN_SIGNED_DOC"

│ DAN_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxlnswnlasdkhtbertssrklgynlgaxwebeutsgaxqdlfflzelslooepthkzsaydtjptansgehdcxeswzrthteowfuolbhnamttkogeoxtekkgdtoemnlpkjlsobwcwnspdbyfeztwtvttsgsmoin
│ DAN_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckgandwlvo
│ DAN_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatkoyaylrtpsotansgylftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckoycsfncsfglfoycsfptpsotansgtlftansgohdcxlnswnlasdkhtbertssrklgynlgaxwebeutsgaxqdlfflzelslooepthkzsaydtjptansgehdcxeswzrthteowfuolbhnamttkogeoxtekkgdtoemnlpkjlsobwcwnspdbyfeztwtvtoybstpsotansgmhdcxykemwkswplndmucyjsahoelatyoedrfslncndyfxmuknaogmvdbsmygrfnpsidtyoycscstpsoiafyhsjtoyaxtpsotansghhdfzencpglghtofnldemchgojoghpthghfvwwtpdfmgsskzscwvtmudpsozmckfgrnvyredicmjyfppttlwlwnglntrklbzmwegrprfppdvaihonemzespmnbyntlrskcknehddaftbz
│ DAN_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatkoyaylstpsotansgylftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzmutngoemftdlcyfshkfmhtmdgdhetdlkrkvtzcknbyytlnpffztntotapkhpwsgrswrdkiosmokpgmlkptknstmhlezsdpsoyabyvepayncpvybbtavlldstwpjocsfptlfdoedn
```

## Building Alice's registry

Set Alice as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
ALICE_REGISTRY=demo/alice-registry.json
frost owner set --registry "$ALICE_REGISTRY" "$ALICE_OWNER_DOC"
frost participant add --registry "$ALICE_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost participant add --registry "$ALICE_REGISTRY" "$CAROL_SIGNED_DOC" Carol
frost participant add --registry "$ALICE_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "$ALICE_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswoyaylrtpsotansgylftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxgwhpoxlfaeltuydpbsidlkgsclhfenzmwfprfdswskgmjtpalnoskiflfldwctfhtansgehdcxtefnbnqdvldevlbwneemkkcezslkmsksftksprtatknsjenlwlmnjnmkwfkbhfhyoybstpsotansgmhdcxiozsahtovozegyiejnmemhseihktflempdjnloldrndiecwztohpnbemmoasiamuoyaxtpsotansghhdfzpswmetlaadkplpcskshtlstsamwnttsgspbkzsfyykfygapscfctctrsgojoaaqznelswyrksgdpkecklfmwlrplbbfskocawkdeylpaetkkzsrhwsqzmofspsdydlmnnnpmrklt"
│   },
│   "participants": {
│     "ur:xid/hdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatksettgufn": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatkoyaylstpsotansgylftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzmutngoemftdlcyfshkfmhtmdgdhetdlkrkvtzcknbyytlnpffztntotapkhpwsgrswrdkiosmokpgmlkptknstmhlezsdpsoyabyvepayncpvybbtavlldstwpjocsfptlfdoedn",
│       "pet_name": "Dan"
│     },
│     "ur:xid/hdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonytoebycs": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonoyaylstpsotansgylftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzahatnewtwegurhlgwmtyrhprvdfyjeotgabzqztpqdsfinnbmdishhlezekiwelgropdolspntaaatjprlrllddmrtosjpihjyveesrhrlwfjlrogrrkwlwtgofptshegoetsbds",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgfdgwiamu": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgoyaylstpsotansgylftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzylwpmtwpsbhphlkgghgtftpyfxdpptgylslrnyparojyclpanntymhoyctwyvlnyuomscmjzkkoncerpclaxzseyecmhrpykytrhmejspystiyhholldsbskdiectljopfaeuyot",
│       "pet_name": "Carol"
│     }
│   }
│ }
```

## Building Bob's registry

Set Bob as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
BOB_REGISTRY=demo/bob-registry.json
frost owner set --registry "$BOB_REGISTRY" "$BOB_OWNER_DOC"
frost participant add --registry "$BOB_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost participant add --registry "$BOB_REGISTRY" "$CAROL_SIGNED_DOC" Carol
frost participant add --registry "$BOB_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "$BOB_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonoyaylrtpsotansgylftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpoycsfncsfglfoycsfptpsotansgtlftansgohdcxatckeheeoetewlatlruttdnetiprcygapfdauortknwyknmegewnvslofxkgehestansgehdcxdkhtwkasidtitijssptiytlagthpfhvasstkkiptnyhsghkbnbreaoryhpoylblyoybstpsotansgmhdcxndwnfwdskejshgmkpyeeetwnwpspzmtarftirtwtemfwihetjkdmfrhpueqdflwyoycscstpsoiafwjlidoyaxtpsotansghhdfzheeynskepsadsetsdsjtcpsswndwprrekkwectvlmtbzbeoeottecknbuypetyurhhpefyztlkbyclrsfeaybkteuotyhlfsgsemfhtibepttyflidvtswfnhsmnwmghlfhyieto"
│   },
│   "participants": {
│     "ur:xid/hdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatksettgufn": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatkoyaylstpsotansgylftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzmutngoemftdlcyfshkfmhtmdgdhetdlkrkvtzcknbyytlnpffztntotapkhpwsgrswrdkiosmokpgmlkptknstmhlezsdpsoyabyvepayncpvybbtavlldstwpjocsfptlfdoedn",
│       "pet_name": "Dan"
│     },
│     "ur:xid/hdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswtpknzswt": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswoyaylstpsotansgylftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzzsinuyleaacplkrehkswynwseydnksbsfnfsvsgehgweiepybzbdbndsbacedebztstlnsvdrkhgbkgwwnididlpaebsjpluhhrokbssialtiepeurkoadlomwwfchtksbryfhhn",
│       "pet_name": "Alice"
│     },
│     "ur:xid/hdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgfdgwiamu": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgoyaylstpsotansgylftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzylwpmtwpsbhphlkgghgtftpyfxdpptgylslrnyparojyclpanntymhoyctwyvlnyuomscmjzkkoncerpclaxzseyecmhrpykytrhmejspystiyhholldsbskdiectljopfaeuyot",
│       "pet_name": "Carol"
│     }
│   }
│ }
```

## Building Carol's registry

Set Carol as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
CAROL_REGISTRY=demo/carol-registry.json
frost owner set --registry "$CAROL_REGISTRY" "$CAROL_OWNER_DOC"
frost participant add --registry "$CAROL_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost participant add --registry "$CAROL_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost participant add --registry "$CAROL_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "$CAROL_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgoyaylrtpsotansgylftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtoycsfncsfglfoycsfptpsotansgtlftansgohdcxqdeeneqdzebyskeelnmngemdrnmhwkwsldkkmwnsgdhkgecszoytfxprlaaogysrtansgehdcxwypmpdwtledetbemahgtveeopmlklussnebbdwiacsdwnldtamwfgyhdpesekiahoybstpsotansgmhdcxotaacpghidtslemnjopdgrykcmwpveattlltsbnbvafdgeqdmwqdwndmspbwpetkoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzadlsldksfmjpspeetodtfmpfykgdlnahfndldeiswmnehkfyptftcxylrneofrsfndgrnyckmejzeoaeamsndiskehssdehhsodkpeonaadlckykrnjorokecabwbwbbmuasgdki"
│   },
│   "participants": {
│     "ur:xid/hdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatksettgufn": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatkoyaylstpsotansgylftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfzmutngoemftdlcyfshkfmhtmdgdhetdlkrkvtzcknbyytlnpffztntotapkhpwsgrswrdkiosmokpgmlkptknstmhlezsdpsoyabyvepayncpvybbtavlldstwpjocsfptlfdoedn",
│       "pet_name": "Dan"
│     },
│     "ur:xid/hdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonytoebycs": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonoyaylstpsotansgylftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzahatnewtwegurhlgwmtyrhprvdfyjeotgabzqztpqdsfinnbmdishhlezekiwelgropdolspntaaatjprlrllddmrtosjpihjyveesrhrlwfjlrogrrkwlwtgofptshegoetsbds",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswtpknzswt": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswoyaylstpsotansgylftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzzsinuyleaacplkrehkswynwseydnksbsfnfsvsgehgweiepybzbdbndsbacedebztstlnsvdrkhgbkgwwnididlpaebsjpluhhrokbssialtiepeurkoadlomwwfchtksbryfhhn",
│       "pet_name": "Alice"
│     }
│   }
│ }
```

## Building Dan's registry

Set Dan as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
DAN_REGISTRY=demo/dan-registry.json
frost owner set --registry "$DAN_REGISTRY" "$DAN_OWNER_DOC"
frost participant add --registry "$DAN_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost participant add --registry "$DAN_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost participant add --registry "$DAN_REGISTRY" "$CAROL_SIGNED_DOC" Carol
cat "$DAN_REGISTRY"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxbeglbtpknewyemwpotztwdhelglobgfebksadketttdygmuynywyhfqdgmmdyatkoyaylrtpsotansgylftanshfhdcxbsmhpeeylygapelklyjnndldvwgabagotsrpbamujpmddklyinlehtjylfimsrsktansgrhdcxkpwydkzmfmsbhputpaykesidwmjtcfenmkcydmdevdietobdskbainkozscxmyckoycsfncsfglfoycsfptpsotansgtlftansgohdcxlnswnlasdkhtbertssrklgynlgaxwebeutsgaxqdlfflzelslooepthkzsaydtjptansgehdcxeswzrthteowfuolbhnamttkogeoxtekkgdtoemnlpkjlsobwcwnspdbyfeztwtvtoybstpsotansgmhdcxykemwkswplndmucyjsahoelatyoedrfslncndyfxmuknaogmvdbsmygrfnpsidtyoycscstpsoiafyhsjtoyaxtpsotansghhdfzencpglghtofnldemchgojoghpthghfvwwtpdfmgsskzscwvtmudpsozmckfgrnvyredicmjyfppttlwlwnglntrklbzmwegrprfppdvaihonemzespmnbyntlrskcknehddaftbz"
│   },
│   "participants": {
│     "ur:xid/hdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonytoebycs": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcximrhtnwlcswsimbywdoemosooxhgyaiydltkmtfyotoerlbbvysfbdyliyihrdonoyaylstpsotansgylftanshfhdcxcprfdkcnrolrwprolfwkbkolcteeamfgkpsktdsknsiodmjesesklgkogwwepebstansgrhdcxjohkolspdtoycejnolemktlbutrtknhnbycxreurtihyihcmbkimmwgmwzutuyhpoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzahatnewtwegurhlgwmtyrhprvdfyjeotgabzqztpqdsfinnbmdishhlezekiwelgropdolspntaaatjprlrllddmrtosjpihjyveesrhrlwfjlrogrrkwlwtgofptshegoetsbds",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswtpknzswt": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxjzroroemvwykcyluaxkpbaztamcestdpvwbdgwolmwkgwmcygydmtsrfjszorlswoyaylstpsotansgylftanshfhdcxmueyaevorstedtrscnhgsgaxrdplfsyasojeuoaachtlkbgosfmwcmfllueheesftansgrhdcxweoldeoywyvywdesotdskbjnjstoqdwfjlsrhylgwkzegelndlskbyoeglmuhybwoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzzsinuyleaacplkrehkswynwseydnksbsfnfsvsgehgweiepybzbdbndsbacedebztstlnsvdrkhgbkgwwnididlpaebsjpluhhrokbssialtiepeurkoadlomwwfchtksbryfhhn",
│       "pet_name": "Alice"
│     },
│     "ur:xid/hdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgfdgwiamu": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxnleybsfpbyasvdaalejzsorfgmpkrlkedmckfrjtlrfdiemusbceprhyssgsfsfgoyaylstpsotansgylftanshfhdcxrdoygunbiygozogdteeohkecjlttpysatbghhdbzlolkgwwsutwpoywkjnwsdepytansgrhdcxhhgdctecytvlprcajnpmpdiyinwzzegldkwnenkteybgvoinhkaaiodirkeeqdbtoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzylwpmtwpsbhphlkgghgtftpyfxdpptgylslrnyparojyclpanntymhoyctwyvlnyuomscmjzkkoncerpclaxzseyecmhrpykytrhmejspystiyhholldsbskdiectljopfaeuyot",
│       "pet_name": "Carol"
│     }
│   }
│ }
```

## Showing Alice's DKG invite (request envelope)

Create a 2-of-3 DKG invite for Bob, Carol, and Dan (from Alice's registry) and format the request envelope to inspect its structure.

```
ALICE_INVITE=$(frost dkg invite show --registry demo/alice-registry.json --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE}" | envelope format

│ request(ARID(c331d11b)) [
│     'body': «"dkgGroupInvite"» [
│         ❰"charter"❱: "This group will authorize new club editions."
│         ❰"minSigners"❱: 2
│         ❰"participant"❱: {
│             {
│                 XID(104e0daa) [
│                     'key': PublicKeys(2cd7072a, SigningPublicKey(104e0daa, SchnorrPublicKey(f2e1a121)), EncapsulationPublicKey(a200e6dc, X25519PublicKey(a200e6dc))) [
│                         'allow': 'All'
│                         'nickname': "Dan"
│                     ]
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         } [
│             "response_arid": ENCRYPTED [
│                 'hasRecipient': SealedMessage
│             ]
│         ]
│         ❰"participant"❱: {
│             {
│                 XID(6ab9dae9) [
│                     'key': PublicKeys(abcb298a, SigningPublicKey(6ab9dae9, SchnorrPublicKey(90b74a5f)), EncapsulationPublicKey(8a89f0a4, X25519PublicKey(8a89f0a4))) [
│                         'allow': 'All'
│                         'nickname': "Bob"
│                     ]
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         } [
│             "response_arid": ENCRYPTED [
│                 'hasRecipient': SealedMessage
│             ]
│         ]
│         ❰"participant"❱: {
│             {
│                 XID(99320f41) [
│                     'key': PublicKeys(e36523a5, SigningPublicKey(99320f41, SchnorrPublicKey(86fdbafa)), EncapsulationPublicKey(8d3574ce, X25519PublicKey(8d3574ce))) [
│                         'allow': 'All'
│                         'nickname': "Carol"
│                     ]
│                 ]
│             } [
│                 'signed': Signature
│             ]
│         } [
│             "response_arid": ENCRYPTED [
│                 'hasRecipient': SealedMessage
│             ]
│         ]
│         ❰"session"❱: ARID(8985148e)
│         ❰"validUntil"❱: 2025-11-22T11:50:57Z
│     ]
│     'date': 2025-11-22T10:50:57Z
│ ]
```

## Showing Alice's sealed DKG invite

Seal the 2-of-3 invite for Bob, Carol, and Dan and format the sealed envelope to view the encrypted recipient entries.

```
ALICE_INVITE_SEALED=$(frost dkg invite show --registry demo/alice-registry.json --sealed --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE_SEALED}" | envelope format
echo "${ALICE_INVITE_SEALED}" | envelope info

│ ENCRYPTED [
│     'hasRecipient': SealedMessage
│     'hasRecipient': SealedMessage
│     'hasRecipient': SealedMessage
│ ]
│ Format: ur:envelope
│ CBOR Size: 2626
│ Description: Gordian Envelope
```

