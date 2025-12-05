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

│ frost-hubert 0.1.0
│ bc-envelope-cli 0.28.0
```

## Configuring storage backend

Set the storage backend for Hubert. Can be 'server', 'dht', 'ipfs', or 'hybrid'.

```
STORAGE=server
TIMEOUT=600
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

│ ALICE_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxfxspgefsfrcwlgbbrotlahsenlluonfsdrptpytdgerdbeghbwpmwzylpaiaehpytansgehdcxghfxdsgmkphtlnpyftnlrnjysorflygafgwzcpfhsglrskkkfdytiennwdtbdpfdverooevo
│ ALICE_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoxktsedl
│ ALICE_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspoyaylrtpsotansgylftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxfxspgefsfrcwlgbbrotlahsenlluonfsdrptpytdgerdbeghbwpmwzylpaiaehpytansgehdcxghfxdsgmkphtlnpyftnlrnjysorflygafgwzcpfhsglrskkkfdytiennwdtbdpfdoybstpsotansgmhdcxonvamnlpdyfsdlhtftlygyttbgmofykscyfxlftpnbmdwypmkspkiocasawpfxwpoyaxtpsotansghhdfzcfmordprdahnrysglfssoehfpelrrfenttbdnnsgemrtjesagevanlmosbbgaaadasmurkhdttcksfaeltjyeylfnynyltdmswmnnlpkykhtwerymegdgtltcsdsatrydlkbdnlf
│ ALICE_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspoyaylstpsotansgylftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzbzfemdcydadwmulewydrtyptnnfgrovybbhgdycpmoaacpsbcmdkclwnadsbzstidrtpcxbzolwtjtfdksnycyhyjsdyjnhddnjnbnkeiooscldkatlyktestaglcecfgrbadirf
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

│ BOB_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxksbgcwbzeoespstetonehtfefpvabbmwwlhghkpyrdgdynjeatylbajtosstkbvatansgehdcxaybnbwbylnsnmelnuoynfxhkhpmovtlnlahfosioyakegrcefssfskhdcmnnrdwdqdjltawn
│ BOB_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxdwttfsnbtdheknrtltnnwfnycetoynehytfsgsfgrdetsfcxnbgdlbmhlbbsierptansgrhdcxeslftpaelarordsnwdsnamchaxjzhkvwglgewzhhzctafhsfsgnlrnsrcfaorohnhlyacflu
│ BOB_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghoyaylrtpsotansgylftanshfhdcxdwttfsnbtdheknrtltnnwfnycetoynehytfsgsfgrdetsfcxnbgdlbmhlbbsierptansgrhdcxeslftpaelarordsnwdsnamchaxjzhkvwglgewzhhzctafhsfsgnlrnsrcfaorohnoycsfncsfglfoycsfptpsotansgtlftansgohdcxksbgcwbzeoespstetonehtfefpvabbmwwlhghkpyrdgdynjeatylbajtosstkbvatansgehdcxaybnbwbylnsnmelnuoynfxhkhpmovtlnlahfosioyakegrcefssfskhdcmnnrdwdoybstpsotansgmhdcxbylkaaimrhtipygriakitlbnferplgutkekkgedsihbyotvdwtwtbaeogsfmeybboycscstpsoiafwjlidoyaxtpsotansghhdfzhdtldnonmwrdsfdsytaxfxjswkfdadrhgerhwfkibsrpveleiejpbkatwezclsjewtwlattlgrmthytlkpneeecandlrfnhffmdwdshdamlbfsmshpderpbkbdjelazsaxldiokk
│ BOB_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghoyaylstpsotansgylftanshfhdcxdwttfsnbtdheknrtltnnwfnycetoynehytfsgsfgrdetsfcxnbgdlbmhlbbsierptansgrhdcxeslftpaelarordsnwdsnamchaxjzhkvwglgewzhhzctafhsfsgnlrnsrcfaorohnoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzlopkrpjehsqzissoaxmtytcarsylrelnfdfwpdlgsgrtbajeestdztwmjytbsebdtbtpbtztsektwfkpsakpmyrdemlpisfnvajlisjoskgdzmcxpdsbietahhiniyiyfhswmncw
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

│ CAROL_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxfevazegejpaxwzbalgmnsrjllettdedwgatkkbwsbdlsbzlydimnsbguimrsmyjstansgehdcxlyaavtnttaqdahieftztctcftpcpeydnamcatilfingsmwryinvefppkvyeohtaemssboxgr
│ CAROL_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxlemhylcxtickdyktdkfnlaptgrjppyztpsgumupyvaoxjkgmamflyattdmwpdpoltansgrhdcxgtnnwkswcndrsajptdoejzjovybbahtlrsjnmouyfylbmdgyrnmwsrdrreiyceceetehfmwd
│ CAROL_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisoyaylrtpsotansgylftanshfhdcxlemhylcxtickdyktdkfnlaptgrjppyztpsgumupyvaoxjkgmamflyattdmwpdpoltansgrhdcxgtnnwkswcndrsajptdoejzjovybbahtlrsjnmouyfylbmdgyrnmwsrdrreiyceceoycsfncsfglfoycsfptpsotansgtlftansgohdcxfevazegejpaxwzbalgmnsrjllettdedwgatkkbwsbdlsbzlydimnsbguimrsmyjstansgehdcxlyaavtnttaqdahieftztctcftpcpeydnamcatilfingsmwryinvefppkvyeohtaeoybstpsotansgmhdcxgtdleynbchhpamrdrotnttnsmkrdotgarsrlaadtoxcwcxuyvlfekiguwyptesspoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzmofmjzsgecmtlumthykbvopylpzobnuoihmeaorywpwtpffneybatssnuoentbldmoenlugesggrfspdlgwtmesodaoxytaazcfsnylbeneysgadaxinbzsozetpjobzdisppeue
│ CAROL_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisoyaylstpsotansgylftanshfhdcxlemhylcxtickdyktdkfnlaptgrjppyztpsgumupyvaoxjkgmamflyattdmwpdpoltansgrhdcxgtnnwkswcndrsajptdoejzjovybbahtlrsjnmouyfylbmdgyrnmwsrdrreiyceceoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzknlebsdagsltlunbwtnbdmrteswmhspawdfdetztzelksftbdnimadwzgholuyrfotwdptcmfrcwgoemaelopltttsweaxwkiykngshluyktlsjycaimgmnyzmdshhjtfrrssgdn
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

│ DAN_PRVKEYS=ur:crypto-prvkeys/lftansgohdcxsbgdwpptiocwkesgwfwebafwrfbdosyapfhpneskmydwylzmwevlcseoonfsfmzotansgehdcxaajzcxosuepymkmnkoylpmrtiyspfzetsnwyamdwpeiehhcphnsglklsahwpkslrlepkftrs
│ DAN_PUBKEYS=ur:crypto-pubkeys/lftanshfhdcxcevwnendykwkcfflfedwhkolbwjkwnlpkkdyuonyttsekespbnmtdmmtwydrbksetansgrhdcxsghhbkttjlinmtmohhjygopsveythguywynlaejesgsendfhnsrduevyeywlbzeedwghrkmn
│ DAN_OWNER_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmoyaylrtpsotansgylftanshfhdcxcevwnendykwkcfflfedwhkolbwjkwnlpkkdyuonyttsekespbnmtdmmtwydrbksetansgrhdcxsghhbkttjlinmtmohhjygopsveythguywynlaejesgsendfhnsrduevyeywlbzeeoycsfncsfgoycscstpsoiafyhsjtlfoycsfptpsotansgtlftansgohdcxsbgdwpptiocwkesgwfwebafwrfbdosyapfhpneskmydwylzmwevlcseoonfsfmzotansgehdcxaajzcxosuepymkmnkoylpmrtiyspfzetsnwyamdwpeiehhcphnsglklsahwpkslroybstpsotansgmhdcxdlhehfcxetaxcmrhgdgthycshyaapdkkditbgryllblfcmttcagwjykeatbgreiyoyaxtpsotansghhdfztesbhsfptknbverssaihmyjpmthlceoytbpyzmhnbeenhstisoueehlrmsjyweemdnfyfgcwlkjpglskbkmwadtplfztgrleamlddtgyhnkibyhdpkishdrdltendsonkgtykkdt
│ DAN_SIGNED_DOC=ur:xid/tpsplftpsplftpsotanshdhdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmoyaylstpsotansgylftanshfhdcxcevwnendykwkcfflfedwhkolbwjkwnlpkkdyuonyttsekespbnmtdmmtwydrbksetansgrhdcxsghhbkttjlinmtmohhjygopsveythguywynlaejesgsendfhnsrduevyeywlbzeeoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfztdtdhffgwsylfeheylosbneylejkcafegamhjsgupkcknygaiatpsrhtnsotytahtscyamrdbycyfgisjpzepmjssgeerovwaajpaokeptzctkzebaihdneelkdnmhwmadkorkhd
```

## Building Alice's registry

Set Alice as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
ALICE_REGISTRY=demo/alice/registry.json
frost registry owner set --registry "$ALICE_REGISTRY" "$ALICE_OWNER_DOC" Alice
frost registry participant add --registry "$ALICE_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost registry participant add --registry "$ALICE_REGISTRY" "$CAROL_SIGNED_DOC" Carol
frost registry participant add --registry "$ALICE_REGISTRY" "$DAN_SIGNED_DOC" Dan
cat "${ALICE_REGISTRY}"

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspoyaylrtpsotansgylftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxfxspgefsfrcwlgbbrotlahsenlluonfsdrptpytdgerdbeghbwpmwzylpaiaehpytansgehdcxghfxdsgmkphtlnpyftnlrnjysorflygafgwzcpfhsglrskkkfdytiennwdtbdpfdoybstpsotansgmhdcxonvamnlpdyfsdlhtftlygyttbgmofykscyfxlftpnbmdwypmkspkiocasawpfxwpoyaxtpsotansghhdfzcfmordprdahnrysglfssoehfpelrrfenttbdnnsgemrtjesagevanlmosbbgaaadasmurkhdttcksfaeltjyeylfnynyltdmswmnnlpkykhtwerymegdgtltcsdsatrydlkbdnlf",
│     "pet_name": "Alice"
│   },
│   "participants": {
│     "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghoyaylstpsotansgylftanshfhdcxdwttfsnbtdheknrtltnnwfnycetoynehytfsgsfgrdetsfcxnbgdlbmhlbbsierptansgrhdcxeslftpaelarordsnwdsnamchaxjzhkvwglgewzhhzctafhsfsgnlrnsrcfaorohnoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzlopkrpjehsqzissoaxmtytcarsylrelnfdfwpdlgsgrtbajeestdztwmjytbsebdtbtpbtztsektwfkpsakpmyrdemlpisfnvajlisjoskgdzmcxpdsbietahhiniyiyfhswmncw",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisoyaylstpsotansgylftanshfhdcxlemhylcxtickdyktdkfnlaptgrjppyztpsgumupyvaoxjkgmamflyattdmwpdpoltansgrhdcxgtnnwkswcndrsajptdoejzjovybbahtlrsjnmouyfylbmdgyrnmwsrdrreiyceceoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzknlebsdagsltlunbwtnbdmrteswmhspawdfdetztzelksftbdnimadwzgholuyrfotwdptcmfrcwgoemaelopltttsweaxwkiykngshluyktlsjycaimgmnyzmdshhjtfrrssgdn",
│       "pet_name": "Carol"
│     },
│     "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmoyaylstpsotansgylftanshfhdcxcevwnendykwkcfflfedwhkolbwjkwnlpkkdyuonyttsekespbnmtdmmtwydrbksetansgrhdcxsghhbkttjlinmtmohhjygopsveythguywynlaejesgsendfhnsrduevyeywlbzeeoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfztdtdhffgwsylfeheylosbneylejkcafegamhjsgupkcknygaiatpsrhtnsotytahtscyamrdbycyfgisjpzepmjssgeerovwaajpaokeptzctkzebaihdneelkdnmhwmadkorkhd",
│       "pet_name": "Dan"
│     }
│   },
│   "groups": {}
│ }
```

## Building Bob's registry

Set Bob as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
BOB_REGISTRY=demo/bob/registry.json
frost registry owner set --registry "$BOB_REGISTRY" "$BOB_OWNER_DOC" Bob
frost registry participant add --registry "$BOB_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost registry participant add --registry "$BOB_REGISTRY" "$CAROL_SIGNED_DOC" Carol
frost registry participant add --registry "$BOB_REGISTRY" "$DAN_SIGNED_DOC" Dan
```

## Building Carol's registry

Set Carol as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
CAROL_REGISTRY=demo/carol/registry.json
frost registry owner set --registry "$CAROL_REGISTRY" "$CAROL_OWNER_DOC" Carol
frost registry participant add --registry "$CAROL_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost registry participant add --registry "$CAROL_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost registry participant add --registry "$CAROL_REGISTRY" "$DAN_SIGNED_DOC" Dan
```

## Building Dan's registry

Set Dan as the registry owner using the private XID document, then add the other three participants with their signed XID documents.

```
DAN_REGISTRY=demo/dan/registry.json
frost registry owner set --registry "$DAN_REGISTRY" "$DAN_OWNER_DOC" Dan
frost registry participant add --registry "$DAN_REGISTRY" "$ALICE_SIGNED_DOC" Alice
frost registry participant add --registry "$DAN_REGISTRY" "$BOB_SIGNED_DOC" Bob
frost registry participant add --registry "$DAN_REGISTRY" "$CAROL_SIGNED_DOC" Carol
```

## Composing Alice's preview DKG invite

Create a 2-of-3 DKG invite for Bob, Carol, and Dan (from Alice's registry) as a preview envelope UR for auditing.

```
ALICE_INVITE_PREVIEW=$(frost dkg coordinator invite --registry demo/alice/registry.json --preview --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE_PREVIEW}" | envelope format

│ {
│     request(ARID(de099bf1)) [
│         'body': «"dkgInvite"» [
│             ❰"charter"❱: "This group will authorize new club editions."
│             ❰"group"❱: ARID(54d115a0)
│             ❰"minSigners"❱: 2
│             ❰"participant"❱: {
│                 {
│                     XID(a84d0bec) [
│                         'key': PublicKeys(b98d0d37, SigningPublicKey(a84d0bec, SchnorrPublicKey(b6ad2f5b)), EncapsulationPublicKey(ee515e9d, X25519PublicKey(ee515e9d))) [
│                             'allow': 'All'
│                             'nickname': "Bob"
│                         ]
│                     ]
│                 } [
│                     'signed': Signature
│                 ]
│             } [
│                 "response_arid": ENCRYPTED [
│                     'hasRecipient': SealedMessage
│                 ]
│             ]
│             ❰"participant"❱: {
│                 {
│                     XID(bbb3100d) [
│                         'key': PublicKeys(c31ba7c2, SigningPublicKey(bbb3100d, SchnorrPublicKey(a5f35cdc)), EncapsulationPublicKey(3fbcfff8, X25519PublicKey(3fbcfff8))) [
│                             'allow': 'All'
│                             'nickname': "Carol"
│                         ]
│                     ]
│                 } [
│                     'signed': Signature
│                 ]
│             } [
│                 "response_arid": ENCRYPTED [
│                     'hasRecipient': SealedMessage
│                 ]
│             ]
│             ❰"participant"❱: {
│                 {
│                     XID(cc871c6e) [
│                         'key': PublicKeys(a5b49935, SigningPublicKey(cc871c6e, SchnorrPublicKey(3ddbbb29)), EncapsulationPublicKey(95754bf5, X25519PublicKey(95754bf5))) [
│                             'allow': 'All'
│                             'nickname': "Dan"
│                         ]
│                     ]
│                 } [
│                     'signed': Signature
│                 ]
│             } [
│                 "response_arid": ENCRYPTED [
│                     'hasRecipient': SealedMessage
│                 ]
│             ]
│             ❰"validUntil"❱: 2025-12-04T19:52:54Z
│         ]
│         'date': 2025-12-04T18:52:54Z
│         'sender': XID(f53e4059) [
│             'key': PublicKeys(a3d556f1, SigningPublicKey(f53e4059, SchnorrPublicKey(6cd09ca0)), EncapsulationPublicKey(622f0841, X25519PublicKey(622f0841))) [
│                 'allow': 'All'
│                 'nickname': "Alice"
│             ]
│         ]
│         'senderContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Composing Alice's sealed DKG invite

Seal the 2-of-3 invite for Bob, Carol, and Dan and format the sealed envelope to view the encrypted recipient entries.

```
ALICE_INVITE_SEALED=$(frost dkg coordinator invite --registry demo/alice/registry.json --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE_SEALED}" | envelope format
echo "${ALICE_INVITE_SEALED}" | envelope info

│ ENCRYPTED [
│     'hasRecipient': SealedMessage
│     'hasRecipient': SealedMessage
│     'hasRecipient': SealedMessage
│ ]
│ Format: ur:envelope
│ CBOR Size: 2619
│ Description: Gordian Envelope
```

## Checking Hubert server availability

Verify the local Hubert server is responding before publishing the invite.

```
frost check --verbose --storage $STORAGE

│ ✓ Hubert server is available at 127.0.0.1:45678 (version 0.3.0)
```

## Sending sealed DKG invite to Hubert

Seal the invite and store it in Hubert using the default server backend; the printed ARID (UR) can be shared out-of-band.

```
ALICE_INVITE_ARID=$(frost dkg coordinator invite --storage $STORAGE --registry demo/alice/registry.json --min-signers 2 --charter "This group will authorize new club editions." Bob Carol Dan)
echo "${ALICE_INVITE_ARID}"

│ ⬆️  ✅ DKG invite: 0s
│ ur:arid/hdcxcemohhlumtrfroonamtymswsaslylrjlurprbsihptenlpleonlbtdbgaywefnmwfzfgdpet
```

## Receiving invite from Hubert as Bob

Retrieve the invite from Hubert using Bob's registry (capturing the envelope), then show the invite details using the cached envelope.

```
BOB_INVITE=$(frost dkg participant receive --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${ALICE_INVITE_ARID}")
frost dkg participant receive --info --registry demo/bob/registry.json "${BOB_INVITE}"

│ ⬇️  ✅ Invite: 0s
│ ur:envelope/lrtansfwlrhkaylkqzatjlsgiatatbvslptowpfwoxrdfgescypkgdsftpcpmkmewndniapedpmtlkdlssamglvdmkzottmnftmdgdssjlbsmhsshffptedwlggltsnladtnmypttngyzoluzcsphgesnememocpcpbefghftscketcsfstkoxgarntpdmmwykfhlnqdpsvabymyhgfloerocnryzmdaidfwlyyabklbcklfhywekprklstowpdywpfgnecxssbnprzsasaoamdlsgahjncphhhdhshkchmkwtfmwzkgtbinhnlnhfbbswkkylynhtnnftswftyldrlfdmjprkrdfxleetmsetvsvdhlmennqdonsakbetpyotghhepsmuasoyfxtnmudapfswoefsztgtlagujskioesehncagmldzsteoyplndolswvtahbanlskkncwvafsghtotefrbntyfeldjzbeeybnhgyawssoesonrttobwsphnryroldsncmjewngtemmdgwistlpststiemcmqdhdrhroztdtctfpaxvlnenlhkhhmwfgihldronseyrodmahmorekicmhnpkvogrhldnsndpqdnsmkyndpzsmkcmhpgedicxdpftttlyuytassjkwpfsjouomtkkeehsimftspmornqzvacehdsnberywsjpfzjsbkonotmkaximvwcmtaihvowlgrlpcejeuodpjemsecdpptoximfyneaotldkkofwbznemkjspshkhynlgaftioashhcndpcyrdpajeamaohsgsbsdeiypyeevsmwaemsaovtlyltpetbiastfhsenevdbktkoysskeltiyiavsythsylwlweglaavoecjtskhemyimasjnhlleinsovdwedpvwuybwiofthywfgumhnypaehdmbytbeoltlkrokbktrhwyetwkahcmlemeprfxotknsptsttwznehtgacnbsjejtaxflsrdwhntypeahasvomyaeluuoisghyktsecuokbwmhkrowfndbwclztoylrvltdpshyhsnsessfhnkplpiodicwgssraonblkgyvynetactfymemnurvdtpcxmtpyrymhtnqdvldrbtwpfgdlwzvdaeleyllemtamolbnatoepecyptsoftdapsecwkfgcemylatofwwsgypsgypsghotolkgkotaotidsehtqzdwtsdeplwfbtisbwdmmhswpaimfpgoskgydedpmtjezeutprtkvalybzjkspdyvyytoygstekijzswayjpgwrlvawnwetifwhnrehdpydlontsjotdmdfxlkjowemttbdspdhtwztpdlnllaimuemufxvdmdtlwtpdckgtmyaoaabzuohyaabblujzcsrkfrbasewecmjsutcsbnoechiykohedluolsrlyngolaykgwkgvecsbbimdkhdvyltpypfmhdpinwliefnaoptmsjybekpcptkistplpsasamdpdsggantlydysfeylebzatbsjponiawlwkwtsbdrssptolptndfyynlddrjyrkvwkoztldjefzpswfdyfzreuyhkhtvszczswsfdvyfwdsbaaxesjevyplwysfkbehnbbznbtdwddsjkchnlfxbzrppllfuyiafeiymnyntotbmsdmfwnbnlzsgywyotjzaopesendtynleoknwsvlstcftnieadiybafydstydwsekolnknrersksjpynnlnljpfymutyzowymubnvapkimenjpemwtcyguuoqdosvevwrsnnredskolsjocedslkjylgplchpeglgmgucwlaatfxcaesrecakemdfsotfxfdvedtpyhkfwsopfjlbawzaxfegdrdjsvyhgdtdnaddkrlbazshyhggwdsaxonwsgepaimiyckdpinbdgwnydlismkytdanbflbkdwgadniytafdcxrowtfxdtjthfotdsropkldwfvlfeurgwldhgjtrhswgepldlgtptatkgiynsrhtkeerlwmoediwzsahycxrftofzytfxecvsaozcrfdkclztahjsluttzmengseoluyabbbzjkytatcecyvsftclsezmspkklfaycxfysppewmisdkonheckjtglbyyagrtdpyksgarklpwsjsptetjerymesrnlimynlplypyserohycllowygwhgvtdscpsaayndckurmunyinplqzftnthflgmsuodnttknsgmklolowlzelafxaajyhnsndksnesrevwlddmosolatchindloybtykiysgossfdmbtcnbzfwjocsdruypllpnsrnrdadzmwlpemdrddrgrtahpmdperorogatbmssngtfzlbztdwdntpcfzodlskdpgofhfsaxpkidlntaghwmbwievduyvaessepkrlidzstdwsjocnyawmeywmcfhksppdpegupeaodychsgmecsrtwddafxsfaxaavavswyrtieetwdcsvdbktdheurqzmovljeltjnahrhrpwefgjyglfmhfrllbzofrpffdfhjnrewtmubwdmkgbdecoehhcpwloymuyautjkluasmspdfdmhmyesgljsdpaeaatsfwtdpsndbagswstyvloszofswmketnvorttleowsmhwysriolegllphlwdrkaoeeecahpldykgplcewffspkbgwpnngltshdfzpebdykhggozmpynsfscfaxbabbrnwmvawmlokgdkwtreiyrnisesesykwnsotemknygmlabscswsmkrtlsbdpmjejysgmunyamcyrylnskrysrahjygtonoerevekpgstkbgmstdglskmnhecavejkytlbntzmrppdmotbcwltiaryjtksndlycfoylbisktykzmgensamrfdtltrnhnaeaefpkppyjehedkimhfbsprkgwttdsovekepfetmsbwpsntjlfztdlbcepronrpzeiyeypmzcinfpsrolaooepymopyjteewsdyprdkpkzekkcebslektkksnjykofentvtinpkswneteqduycncxemknpkrntduyfgqzvwjzzttocyuoglbghsemwzrkjpnywsehaxrnyamhwkhnghmdimrkoyjkgyeekpsrlbnniaadlnuyjtchisvlsohghejsctdtkgeordcfkpecjncnmololohpnyiemsgttskotbytylmycwjlmwaejohklubbdrmesbqzjlwdgdlymyvytkeebtvevlvscsuywltnmwdpttampdiarpfxlemnlkcpwmoyinndetrorpsbkphhzoknisoxjoykcxwmghmklslrptrnnblumovocaehespyadcyhhpmmsfsytyaiholkiptpftaltlsgwdnpyadswrssktdwsynsruoahvdbgsoehetmsmuoliscktsdsdreswdmkrlmujlrhskdmbwemtpsstneoskcxlnsbqddadwfzsotyenfhpkfzckwfcpecatcnhkiyfztpuocsfngstaaahyplndnsolwybejtfpqdmuonmtyalgfrimvtylntmennttnljerkehnbtnsnrtpdvstsmumtmerfcxfxlssrgrfscxwyzoatuourvefzfxgshnnsfwchemolftstutoyghrhjzwmvlhsbskpskftspayrkeczmaeeybktdwpwffpsrhhosbdchdtdlswaypszcuyfhehmygyhkiyknmkjzlosrhgwskskbbklfhhwluesegygmvwlatoztjyiecnrfpdckktkgbenbcyltayeosprpvwvebdktkecfhtrpzcnstochmkbwsbnsntfldslbykmstydivooyfwfxgauotadkmhvtzmjkfptoennemkaepflkeeqdvttdwzleosbgvetdgdgyesihwkeordjyhnglmyrdpmbyrpnemkemdrtbdaonahwfsawphfoyylhnbngmskrfmkrdnnlphgfhyabgbbaadmfnoycxglzeemhfrywtoeonvsgwidredapyynfhrpfsgwwlwlehaxaerksnztcxjtcyhkmomwnteyldlsktcaennstigocaeyzmcxwkkgtszsutchoxuyrpvldpnyrdihgrmeaxatsgaheysrjniebzdmfxwnfxpyinpacaurbsaejeiakicxdartlddifpaylrzmwzgmuyoxtafnkbfnlnrtgyoykkbkmoiyvonysscxihlgfyemtbkerdgefpsbtaionsgutblswsfpswstlecawtgshgjsottpoefpknwlgahgwyqzgdzcfmutrsatbkrowmjeynaecninctctynhddatansfphdcxktnlcwdavygadwwdhtayregoenglectabdlpjzmeoyjztysglbtykidmurgyrpjyoyahtpsotansgulftansfwlshddacwcmsghsdyhscygarprewnbecyrdhfcatyvdeodrtykgynmyditdihbdeckswsasonbntymopygslkgrlkkkgstdpaykrtgtyaahgdluwsvyhlpdcxwztdlkdmkprotbhgueyktansgrhdcxwytppaztbsfmemdezebsamolcebtcyheptcmcnpynlvwbgmubywfynfyrycnlpaeoyahtpsotansgulftansfwlshddaaezmbklooeoshkgdcmeyztrpftjkbghtensfoevyurzefsjlbdyaetlgeoldwtimqzfrjycxylgskskoqztegypaoerlzebgwlhkgdaahlhpzefwdrsfmhhptsolmthfgrcpmdtansgrhdcxdpecwnvlbncwlbhllnztplisspwzhfmsghprgotsteontezmwmrhdiecwyclkkbeoyahtpsotansgulftansfwlshddatdgeknchtnwkgewzkbhtlegrosgoprhghnmsdrbtqzhfcfnsvasgtkueqdimgyttdwrhfdgeswgsfdrysaghemjessberppsldmegdyttycpregezcfrdaimeodajnpriobzostansgrhdcxtpaytytlbsdnguuthsrlcmtpbyvadndiflvyswmnqdspzolopllgyamdchbyfsemfwzsztrt
│ Charter: This group will authorize new club editions.
│ Min signers: 2
│ Coordinator: Alice
│ Participants: * Bob, Carol, Dan
```

## Composing Bob's preview Round 1 response

Preview the response envelope structure before posting. This shows the DKG Round 1 package and group metadata.

```
BOB_RESPONSE_PREVIEW=$(frost dkg participant round1 --preview --registry demo/bob/registry.json "${BOB_INVITE}")
echo "${BOB_RESPONSE_PREVIEW}" | envelope format

│ {
│     response(ARID(c359ba85)) [
│         'recipientContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│         'result': '' [
│             'isA': "dkgRound1Response"
│             "group": ARID(defe451b)
│             "participant": XID(a84d0bec)
│             "response_arid": ARID(75a48f59)
│             "round1_package": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"commitment":["63afd290447b5ea4d1efc3f57d794768e96a0ccb0ea0d5e5a2bfca2c061c3d06","772047d685e2740feab860035d08d26f3c2ee3636133f26e99b875e7bddd0f80"],"proof_of_knowledge":"53745652d57937b26cc86e401a6735f57482a87ca83559cb6bfbcad21f0bc07693e9f04952ba467b696ecbbd31db2adc1db7d49e46f8ee5218a272d181a0be0e"})
│         ]
│         'sender': XID(a84d0bec) [
│             'key': PublicKeys(b98d0d37, SigningPublicKey(a84d0bec, SchnorrPublicKey(b6ad2f5b)), EncapsulationPublicKey(ee515e9d, X25519PublicKey(ee515e9d))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│         'senderContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Composing Bob's sealed Round 1 response

The sealed response is encrypted to a single recipient (Alice, the coordinator).

```
BOB_RESPONSE_SEALED=$(frost dkg participant round1 --registry demo/bob/registry.json "${BOB_INVITE}")
echo "${BOB_RESPONSE_SEALED}" | envelope format

│ ENCRYPTED [
│     'hasRecipient': SealedMessage
│ ]
```

## Bob posts Round 1 response

Post Bob's sealed Round 1 response to Hubert using the cached invite envelope.

```
frost dkg participant round1 --verbose --storage $STORAGE --registry demo/bob/registry.json "${BOB_INVITE}"

│ [2025-12-04T18:52:54.320Z] Starting server put operation
│ [2025-12-04T18:52:54.320Z] Sending PUT request to server
│ [2025-12-04T18:52:54.321Z] Server put operation completed
│ ⬆️  ✅ Round 1 Response: 0s
```

## Carol and Dan post Round 1 responses

Carol and Dan accept the invite from Hubert using their registries, posting their Round 1 responses to Hubert.

```
frost dkg participant round1 --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/carol/registry.json "${ALICE_INVITE_ARID}"
frost dkg participant round1 --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/dan/registry.json "${ALICE_INVITE_ARID}"

│ [2025-12-04T18:52:54.330Z] Starting server get operation
│ [2025-12-04T18:52:54.330Z] Polling server for value
│ [2025-12-04T18:52:54.331Z] Value found on server
│ [2025-12-04T18:52:54.331Z] Server get operation completed
│ ⬇️  ✅ Invite: 0s
│ [2025-12-04T18:52:54.333Z] Starting server put operation
│ [2025-12-04T18:52:54.333Z] Sending PUT request to server
│ [2025-12-04T18:52:54.334Z] Server put operation completed
│ ⬆️  ✅ Round 1 Response: 0s
│ [2025-12-04T18:52:54.344Z] Starting server get operation
│ [2025-12-04T18:52:54.344Z] Polling server for value
│ [2025-12-04T18:52:54.345Z] Value found on server
│ [2025-12-04T18:52:54.345Z] Server get operation completed
│ ⬇️  ✅ Invite: 0s
│ [2025-12-04T18:52:54.347Z] Starting server put operation
│ [2025-12-04T18:52:54.347Z] Sending PUT request to server
│ [2025-12-04T18:52:54.347Z] Server put operation completed
│ ⬆️  ✅ Round 1 Response: 0s
```

## Inspecting Alice's registry after sending invite

Alice's registry now contains the group record with pending_requests listing the response ARIDs for each participant.

```
jq . demo/alice/registry.json

│ {
│   "owner": {
│     "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspoyaylrtpsotansgylftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoycsfncsfgoycscstpsoihfpjziniaihlfoycsfptpsotansgtlftansgohdcxfxspgefsfrcwlgbbrotlahsenlluonfsdrptpytdgerdbeghbwpmwzylpaiaehpytansgehdcxghfxdsgmkphtlnpyftnlrnjysorflygafgwzcpfhsglrskkkfdytiennwdtbdpfdoybstpsotansgmhdcxonvamnlpdyfsdlhtftlygyttbgmofykscyfxlftpnbmdwypmkspkiocasawpfxwpoyaxtpsotansghhdfzcfmordprdahnrysglfssoehfpelrrfenttbdnnsgemrtjesagevanlmosbbgaaadasmurkhdttcksfaeltjyeylfnynyltdmswmnnlpkykhtwerymegdgtltcsdsatrydlkbdnlf",
│     "pet_name": "Alice"
│   },
│   "participants": {
│     "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghoyaylstpsotansgylftanshfhdcxdwttfsnbtdheknrtltnnwfnycetoynehytfsgsfgrdetsfcxnbgdlbmhlbbsierptansgrhdcxeslftpaelarordsnwdsnamchaxjzhkvwglgewzhhzctafhsfsgnlrnsrcfaorohnoycsfncsfgoycscstpsoiafwjlidoyaxtpsotansghhdfzlopkrpjehsqzissoaxmtytcarsylrelnfdfwpdlgsgrtbajeestdztwmjytbsebdtbtpbtztsektwfkpsakpmyrdemlpisfnvajlisjoskgdzmcxpdsbietahhiniyiyfhswmncw",
│       "pet_name": "Bob"
│     },
│     "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisoyaylstpsotansgylftanshfhdcxlemhylcxtickdyktdkfnlaptgrjppyztpsgumupyvaoxjkgmamflyattdmwpdpoltansgrhdcxgtnnwkswcndrsajptdoejzjovybbahtlrsjnmouyfylbmdgyrnmwsrdrreiyceceoycsfncsfgoycscstpsoihfxhsjpjljzoyaxtpsotansghhdfzknlebsdagsltlunbwtnbdmrteswmhspawdfdetztzelksftbdnimadwzgholuyrfotwdptcmfrcwgoemaelopltttsweaxwkiykngshluyktlsjycaimgmnyzmdshhjtfrrssgdn",
│       "pet_name": "Carol"
│     },
│     "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│       "xid_document": "ur:xid/tpsplftpsplftpsotanshdhdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmoyaylstpsotansgylftanshfhdcxcevwnendykwkcfflfedwhkolbwjkwnlpkkdyuonyttsekespbnmtdmmtwydrbksetansgrhdcxsghhbkttjlinmtmohhjygopsveythguywynlaejesgsendfhnsrduevyeywlbzeeoycsfncsfgoycscstpsoiafyhsjtoyaxtpsotansghhdfztdtdhffgwsylfeheylosbneylejkcafegamhjsgupkcknygaiatpsrhtnsotytahtscyamrdbycyfgisjpzepmjssgeerovwaajpaokeptzctkzebaihdneelkdnmhwmadkorkhd",
│       "pet_name": "Dan"
│     }
│   },
│   "groups": {
│     "ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue": {
│       "charter": "This group will authorize new club editions.",
│       "min_signers": 2,
│       "coordinator": "ur:xid/hdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspeehndpcf",
│       "participants": [
│         "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl",
│         "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt",
│         "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo"
│       ],
│       "pending_requests": {
│         "requests": [
│           {
│             "participant": "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl",
│             "collect_from_arid": "ur:arid/hdcxtljztaaaurvlfllewlgtenbbvdbayauopyaswpzsbeinmkclaomnkkkbencwpfvwstntgofp"
│           },
│           {
│             "participant": "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt",
│             "collect_from_arid": "ur:arid/hdcxbtzogutdgojljsbeylmunnsarnaelrrofsuohkdnpfynzewlbafgdtldcekgmwztsfjkglvw"
│           },
│           {
│             "participant": "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo",
│             "collect_from_arid": "ur:arid/hdcxmybtkbissbgwztstlkeewecnhsfztlttiedrsnlpknztlpimgwzshspdlysgswinjegaadsg"
│           }
│         ]
│       }
│     }
│   }
│ }
```

## Alice collects Round 1 responses and sends Round 2 requests

Alice fetches each participant's sealed response from Hubert, saves the Round 1 packages, and immediately posts individualized Round 2 requests. One Round 2 request is previewed while posting.

```
# Get the group ID from Alice's registry
ALICE_GROUP_ID=$(jq -r '.groups | keys[0]' demo/alice/registry.json)
echo "Group ID: ${ALICE_GROUP_ID}"

# Collect Round 1 responses and dispatch Round 2 (with a preview of one request)
ROUND1_PREVIEW=$(frost dkg coordinator round1 --preview --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/alice/registry.json "${ALICE_GROUP_ID}" | tee /dev/stderr | tail -n1)
echo "${ROUND1_PREVIEW}" | envelope format

│ Group ID: ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue
│ Collecting Round 1 responses from 3 participants...
│ [2025-12-04T18:52:54.367Z] Starting server get operation
│ [2025-12-04T18:52:54.367Z] Polling server for value
│ [2025-12-04T18:52:54.367Z] Value found on server
│ [2025-12-04T18:52:54.367Z] Server get operation completed
│ ⬇️  ✅ Bob: 0s
│ [2025-12-04T18:52:54.368Z] Starting server get operation
│ [2025-12-04T18:52:54.368Z] Polling server for value
│ [2025-12-04T18:52:54.368Z] Value found on server
│ ⬇️[2025-12-04T18:52:54.368Z] Server get operation completed
│   ✅ Carol: 0s
│ [2025-12-04T18:52:54.369Z] Starting server get operation
│ [2025-12-04T18:52:54.369Z] Polling server for value
│ [2025-12-04T18:52:54.369Z] Value found on server
│ ⬇️  ✅ [2025-12-04T18:52:54.369Z] Server get operation completed
│ Dan: 0s
│ Sending Round 2 requests to 3 participants...
│ [2025-12-04T18:52:54.371Z] Starting server put operation
│ [2025-12-04T18:52:54.371Z] Sending PUT request to server
│ ⬆️  ✅ [2025-12-04T18:52:54.372Z] Server put operation completed
│ Bob: 0s
│ [2025-12-04T18:52:54.372Z] Starting server put operation
│ [2025-12-04T18:52:54.372Z] Sending PUT request to server
│ ⬆️  ✅ Carol: 0s
│ [2025-12-04T18:52:54.373Z] Server put operation completed
│ [2025-12-04T18:52:54.373Z] Starting server put operation
│ [2025-12-04T18:52:54.373Z] Sending PUT request to server
│ [2025-12-04T18:52:54.377Z] Server put operation completed
│ ⬆️  ✅ Dan: 0s
│ # Round 2 preview for Bob
│
│ Collected 3 Round 1 packages to demo/alice/group-state/defe451bcbb099e6488201a36ba764dd4832880d8c4b339bb220fb4ce6c87ef4/collected_round1.json and sent 3 Round 2 requests.
│ ur:envelope/lftpsplrtpsotansfytansgshdcxosdiwftphgnylezmvswsrfchimkbaaeetoltbewzcsneotfzwzfeqzpachwtwtluoycsielntpsotansfginiejeiogmjlkpjtieeyoytpsotansfljnjpjlkpjtieehgdhsiajehsioihlftpsotaadamhkadjnkgcpisihhsieihjpcpftkgcpkoihjpjkinjljtcpftdydwcpiainjoisihjpjkkpinjyihcpftcpfggmgwgughdpfefyeyececehesdpgufdfpeceheydpkoehcpkidwcpiajljnjninjyjnihjtjycpfthpcpeoiyiyehiheyenemeciaiyeciahsemieeeeciyiaeodyieihiyihetidecendyiaehdyiedyhsideeeeemiaehdyhseyehhseeiadyiyememesiydyhseeesenemehihcpdwcpdyihieidiyiydyemeehsesenihhseyehdyesiyeyidehiaemihiaiaidiedyiahsesesiyiyesecesenihhseeeeeceeesidesieetideohsidenesdyehehecechsencphldwcpjojpjljliyhejliyhejejtjlktjzihieioihcpftcpeehseniyetideciheeihhsenemecemeeehiydyeyieeoenihidiaiyidiaieiaenehidiyeoeceyecendyihdyeseniaeyiyenihiaehecihideyeyeyemeciedydyecetemidiaeyemesiaechseeiaecideteneteteeeyiyeciyhsesiedydyetihideyiaeshsetesehiyehiheheeiaeyeeidihideoiaendyesdyiyiaihieiyenhsdyemcpkioytpsojejohsjpjyiniainjohsjtjytpsotanshdhdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghoytpsotansfljzjpihjkjojljtjkihfpjpinietpsotansgshdcxbzaslavyhgaaplwfluswskoerkdpykweolfsnbbwdyialtcedauotdiyqdrlaschoytpsotansfljnjpjlkpjtieehgdhsiajehsioihlftpsotaadamhkadjnkgcpisihhsieihjpcpftkgcpkoihjpjkinjljtcpftdydwcpiainjoisihjpjkkpinjyihcpftcpfggmgwgughdpfefyeyececehesdpgufdfpeceheydpkoehcpkidwcpiajljnjninjyjnihjtjycpfthpcpetieeeeyenetiyieeshsiyeheyeyeteheseoieihieehihieececiydyesideyiyieeneydyemhseeeseeihemihemeneteteniaiaesiddyemenidiyenemeoetemiacpdwcpeciheyeseneeiyeseseseehsesiahseciaenihiyeneeenieeteheoesesiyieemeoemiaeohsidetecdyiheeiaiaememeceoecideoeeiydyidesetecenetihemiecphldwcpjojpjljliyhejliyhejejtjlktjzihieioihcpftcpetdyendyiaihiydyeoihiyehemendyiyeoieeteoieihemeheoiyenemeoeoehdyeoieeoihiaiaetideciyieecideteoemenhsenesiaeneciyieideeieiaieidemeyiyetiyeeeteciaeniyidehiaesetiaeodyhseoieeheeeseteeeyeyhshsecececeoeeeseteyhsemecdyeheeemeyeoiyetieetieideeeoeteteceyeodyendydycpkioytpsojejohsjpjyiniainjohsjtjytpsotanshdhdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmoytpsotansflihiojpjlkpjotpsotansgshdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkoytpsotansfljnjpjlkpjtieehgdhsiajehsioihlftpsotaadamhkadjnkgcpisihhsieihjpcpftkgcpkoihjpjkinjljtcpftdydwcpiainjoisihjpjkkpinjyihcpftcpfggmgwgughdpfefyeyececehesdpgufdfpeceheydpkoehcpkidwcpiajljnjninjyjnihjtjycpfthpcpiaeyeceoecihieeheyetetemiaeeididiyeyeniaecehihecieehieeyesetetdyeoeyihehieehehemecihemeeeeeoeyeshseyiyeyidemeniaidememdyeoehiyescpdwcpenenesidemehihemdyieeseyiedyhseohshsidenihiyiyeyieihiaiyechseheoenesesetetemihihihiedyehdyeyhsesememeeetiddyieideeidihemeyececeycphldwcpjojpjljliyhejliyhejejtjlktjzihieioihcpftcpiaechsenenihiaeoeniaemieieeyiaihihidemeteeeodydyiyesenemeteeideedyetesihieididenetenidhsenecemeseyidesenidiheoecesiaeneyiaemeneceheeieieiyiaeniheyhsiaieieiahseheodyhsdyeoesiheyiahsemenieihemeyeseoeeieeseyeteyeoiaeyeheteeieideeetiyiyehihetehesenenesenecdyehcpkioytpsojejohsjpjyiniainjohsjtjytpsotanshdhdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisoycsimlftansfwlrhdfpeeluwdtphtvlbtrfssrhpdsfetzmwtpeqztpjeqddtaewefdntvslgchfdsotechmetbjpjlbdpyzoethkcafttnroastpwnpylecxahltbggrgmaerhtoimsbhtvorpylgsvyghzcqzhtdntidapahyutwtgddrlowydpenjzlufebabbuoytrhethkrhhddatansfphdcxrkldgydnfgjtsechgarfrnaygujtchdrfraalulgrszeiylfrhbkyavdwnmdqzynoyahtpsotansgulftansfwlshddaskmusfsnrebewdlpcplsmokkdpjzzcosgupshkvsfnjkbnjptlesbtbeievtpectwpeelptevegskpbyqzzohldsckldwffgpftigddetpwkcmzsfdhhrdehpaztpkwyfzpynbtansgrhdcxcxtngmnsenvaynfnrhbnfehphfdmwdrsrhasecdyrhpasrherfsnwpasimpdrddyoycsinlftpsotanshdhdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspoyaylstpsotansgylftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoycsfncsfgoycscstpsoihfpjziniaihoyaxtpsotansghhdfzluknzoisuyeshpjetiimcpemjlwlmerpynbtjpyamdyksfbnpewydmynsnwyjzldjtglhdzsztfdkpwdmostsbrdesfwbswlcyadwphkeygtksfwvyoxeywmwptdihylmusgcllo
│ {
│     request(ARID(a727f3d8)) [
│         'body': «"dkgRound2"» [
│             ❰"group"❱: ARID(defe451b)
│             ❰"responseArid"❱: ARID(150980e1)
│             ❰"round1Package"❱: JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"commitment":["3ff1e2675cf5ca7d45fc30defe8b560c10d0ab447c10a21a4c0f779f0a49671e","0edbff074a96ea2109f2b1c7eccbd0ca99ff9596ea44549b9d8b3ab6901155a6"],"proof_of_knowledge":"4a6f8b5e4ea675741f02d36ebcfbcdc61bf352560e096c2f6ec15eb22275d00587bc279c5a4c5b868842f5fa9d008eb2c9a891f1e14c24beb3c6090fcedf6a07"}) [
│                 "participant": XID(a84d0bec)
│             ]
│             ❰"round1Package"❱: JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"commitment":["8d4268fd9af1228193ded1ed55f09b2fd6207a494e7e76886cc9b076bf67387c","5e2964f9994a9ca5c6ef646d81399fd737c3ab850e4cc77535b34f0b98568e7d"],"proof_of_knowledge":"8060cef03ef1760f3d83de713f6733103d3ecc8b5fd5b8376a69c65fdb4dcdb72f8f485c6fb1c98c30a3d1498422aa55534982a75014723f8d8db43885230600"}) [
│                 "participant": XID(cc871c6e)
│             ]
│             ❰"round1Package"❱: JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"commitment":["c2535ed12887c4bbf26c51e5d1d2988032e1d1175e744329a2f2b76cb77031f9","669b71e70d92d0a3aab6eff2decf5a13699887eeed0102a97748b0db4be72552"],"proof_of_knowledge":"c5a66ec36c7dd2ceeb784300f96784b4089edbb686ba65792b96be359c62c76514ddfc6e2acddca130a039e2ca76de72934d92823c2184db48ff1e8196696501"}) [
│                 "participant": XID(bbb3100d)
│             ]
│         ]
│         'sender': XID(f53e4059) [
│             'key': PublicKeys(a3d556f1, SigningPublicKey(f53e4059, SchnorrPublicKey(6cd09ca0)), EncapsulationPublicKey(622f0841, X25519PublicKey(622f0841))) [
│                 'allow': 'All'
│                 'nickname': "Alice"
│             ]
│         ]
│         'senderContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Inspecting collected Round 1 packages

The collected Round 1 packages are stored in Alice's group-state directory, ready for Round 2 processing.

```
jq . demo/alice/group-state/*/collected_round1.json

│ {
│   "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│     "commitment": [
│       "3ff1e2675cf5ca7d45fc30defe8b560c10d0ab447c10a21a4c0f779f0a49671e",
│       "0edbff074a96ea2109f2b1c7eccbd0ca99ff9596ea44549b9d8b3ab6901155a6"
│     ],
│     "header": {
│       "ciphersuite": "FROST-ED25519-SHA512-v1",
│       "version": 0
│     },
│     "proof_of_knowledge": "4a6f8b5e4ea675741f02d36ebcfbcdc61bf352560e096c2f6ec15eb22275d00587bc279c5a4c5b868842f5fa9d008eb2c9a891f1e14c24beb3c6090fcedf6a07"
│   },
│   "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│     "commitment": [
│       "c2535ed12887c4bbf26c51e5d1d2988032e1d1175e744329a2f2b76cb77031f9",
│       "669b71e70d92d0a3aab6eff2decf5a13699887eeed0102a97748b0db4be72552"
│     ],
│     "header": {
│       "ciphersuite": "FROST-ED25519-SHA512-v1",
│       "version": 0
│     },
│     "proof_of_knowledge": "c5a66ec36c7dd2ceeb784300f96784b4089edbb686ba65792b96be359c62c76514ddfc6e2acddca130a039e2ca76de72934d92823c2184db48ff1e8196696501"
│   },
│   "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│     "commitment": [
│       "8d4268fd9af1228193ded1ed55f09b2fd6207a494e7e76886cc9b076bf67387c",
│       "5e2964f9994a9ca5c6ef646d81399fd737c3ab850e4cc77535b34f0b98568e7d"
│     ],
│     "header": {
│       "ciphersuite": "FROST-ED25519-SHA512-v1",
│       "version": 0
│     },
│     "proof_of_knowledge": "8060cef03ef1760f3d83de713f6733103d3ecc8b5fd5b8376a69c65fdb4dcdb72f8f485c6fb1c98c30a3d1498422aa55534982a75014723f8d8db43885230600"
│   }
│ }
```

## Inspecting Alice's registry after Round 2 dispatch

Alice's registry now has pending_requests with the coordinator's collection ARIDs for Round 2 responses.

```
jq '.groups' demo/alice/registry.json

│ {
│   "ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue": {
│     "charter": "This group will authorize new club editions.",
│     "min_signers": 2,
│     "coordinator": "ur:xid/hdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspeehndpcf",
│     "participants": [
│       "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl",
│       "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt",
│       "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo"
│     ],
│     "pending_requests": {
│       "requests": [
│         {
│           "participant": "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl",
│           "collect_from_arid": "ur:arid/hdcxbzaslavyhgaaplwfluswskoerkdpykweolfsnbbwdyialtcedauotdiyqdrlaschfemtfskg"
│         },
│         {
│           "participant": "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt",
│           "collect_from_arid": "ur:arid/hdcxdnpaltmktkrybbcfplnseoutprnllynluosnntwecfktrkoshsbsoxcllblfnbmwpdlfdsla"
│         },
│         {
│           "participant": "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo",
│           "collect_from_arid": "ur:arid/hdcxsrqzsbssfloykersdwseykfmamgrdydsfwksspjkbdeclpkoadhsehiddkckqzztdmfywsia"
│         }
│       ]
│     }
│   }
│ }
```

## Checking Bob's listening ARID for Round 2

Bob's registry still records the ARID where Alice posted the Round 2 request.

```
jq '.groups[].listening_at_arid' demo/bob/registry.json

│ "ur:arid/hdcxyalblbghvohplnmodrlrfdvehtludrndemursrrofmwmzocplywzqdvlprielevsiyzthyim"
```

## Bob responds to Round 2 request

Bob fetches the Round 2 request, runs FROST DKG part2 with his Round 1 secret and all Round 1 packages, generates Round 2 packages, and prints the preview response envelope structure (no post).

```
BOB_GROUP_ID=$(jq -r '.groups | keys[0]' demo/bob/registry.json)
frost dkg participant round2 --preview --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_GROUP_ID}" | envelope format

│ ⬇️  ✅ Round 2 request: 0s
│ {
│     response(ARID(a727f3d8)) [
│         'recipientContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│         'result': '' [
│             'isA': "dkgRound2Response"
│             "group": ARID(defe451b)
│             "participant": XID(a84d0bec)
│             "response_arid": ARID(588de249)
│             "round2Package": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"signing_share":"74baab4c2449001a795545d126b36ce3316302ad8a99adce63fa179a2e70cf00"}) [
│                 "recipient": XID(bbb3100d)
│             ]
│             "round2Package": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"signing_share":"ce1edcda3dc204a64b7609f50beeaaecca2e6a13b5ed913333cf59b568165808"}) [
│                 "recipient": XID(cc871c6e)
│             ]
│         ]
│         'sender': XID(a84d0bec) [
│             'key': PublicKeys(b98d0d37, SigningPublicKey(a84d0bec, SchnorrPublicKey(b6ad2f5b)), EncapsulationPublicKey(ee515e9d, X25519PublicKey(ee515e9d))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Bob posts Round 2 response

Bob posts the sealed Round 2 response to the coordinator (no preview output).

```
BOB_GROUP_ID=$(jq -r '.groups | keys[0]' demo/bob/registry.json)
frost dkg participant round2 --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_GROUP_ID}"

│ Fetching Round 2 request from Hubert...
│ [2025-12-04T18:52:54.427Z] Starting server get operation
│ [2025-12-04T18:52:54.427Z] Polling server for value
│ [2025-12-04T18:52:54.428Z] Value found on server
│ [2025-12-04T18:52:54.428Z] Server get operation completed
│ ⬇️  ✅ Round 2 request: 0s
│ Received 2 Round 1 packages. Running DKG part2...
│ Generated 2 Round 2 packages.
│ [2025-12-04T18:52:54.431Z] Starting server put operation
│ [2025-12-04T18:52:54.431Z] Sending PUT request to server
│ [2025-12-04T18:52:54.431Z] Server put operation completed
│ ⬆️  ✅ Round 2 Response: 0s
│ Posted Round 2 response to ur:arid/hdcxbzaslavyhgaaplwfluswskoerkdpykweolfsnbbwdyialtcedauotdiyqdrlaschfemtfskg
```

## Carol responds to Round 2 request

Carol processes the Round 2 request with her Round 1 secret and all Round 1 packages, generates Round 2 packages, and posts them back to the coordinator.

```
CAROL_GROUP_ID=$(jq -r '.groups | keys[0]' demo/carol/registry.json)
frost dkg participant round2 --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/carol/registry.json "${CAROL_GROUP_ID}"

│ Fetching Round 2 request from Hubert...
│ [2025-12-04T18:52:54.446Z] Starting server get operation
│ [2025-12-04T18:52:54.446Z] Polling server for value
│ [2025-12-04T18:52:54.447Z] Value found on server
│ [2025-12-04T18:52:54.447Z] Server get operation completed
│ ⬇️  ✅ Round 2 request: 0s
│ Received 2 Round 1 packages. Running DKG part2...
│ Generated 2 Round 2 packages.
│ [2025-12-04T18:52:54.448Z] Starting server put operation
│ [2025-12-04T18:52:54.448Z] Sending PUT request to server
│ [2025-12-04T18:52:54.449Z] Server put operation completed
│ ⬆️  ✅ Round 2 Response: 0s
│ Posted Round 2 response to ur:arid/hdcxdnpaltmktkrybbcfplnseoutprnllynluosnntwecfktrkoshsbsoxcllblfnbmwpdlfdsla
```

## Dan responds to Round 2 request

Dan processes the Round 2 request with his Round 1 secret and all Round 1 packages, generates Round 2 packages, and posts them back to the coordinator.

```
DAN_GROUP_ID=$(jq -r '.groups | keys[0]' demo/dan/registry.json)
frost dkg participant round2 --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/dan/registry.json "${DAN_GROUP_ID}"

│ Fetching Round 2 request from Hubert...
│ [2025-12-04T18:52:54.462Z] Starting server get operation
│ [2025-12-04T18:52:54.462Z] Polling server for value
│ [2025-12-04T18:52:54.463Z] Value found on server
│ [2025-12-04T18:52:54.463Z] Server get operation completed
│ ⬇️  ✅ Round 2 request: 0s
│ Received 2 Round 1 packages. Running DKG part2...
│ Generated 2 Round 2 packages.
│ [2025-12-04T18:52:54.464Z] Starting server put operation
│ [2025-12-04T18:52:54.464Z] Sending PUT request to server
│ [2025-12-04T18:52:54.465Z] Server put operation completed
│ ⬆️  ✅ Round 2 Response: 0s
│ Posted Round 2 response to ur:arid/hdcxsrqzsbssfloykersdwseykfmamgrdydsfwksspjkbdeclpkoadhsehiddkckqzztdmfywsia
```

## Alice collects Round 2 responses and sends finalize packages

Alice fetches each participant's Round 2 response from Hubert, saves the Round 2 packages, and immediately posts finalize requests. One finalize request is previewed while posting.

```
# Get the group ID from Alice's registry
ALICE_GROUP_ID=$(jq -r '.groups | keys[0]' demo/alice/registry.json)
echo "Group ID: ${ALICE_GROUP_ID}"

# Collect Round 2 responses and dispatch finalize (with a preview of one request)
ROUND2_PREVIEW=$(frost dkg coordinator round2 --preview --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/alice/registry.json "${ALICE_GROUP_ID}" | tee /dev/stderr | tail -n1)
echo "${ROUND2_PREVIEW}" | envelope format

│ Group ID: ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue
│ Collecting Round 2 responses from 3 participants...
│ Bob...
│ [2025-12-04T18:52:54.477Z] Starting server get operation
│ [2025-12-04T18:52:54.477Z] Polling server for value
│ [2025-12-04T18:52:54.478Z] Value found on server
│ [2025-12-04T18:52:54.478Z] Server get operation completed
│ Carol...
│ [2025-12-04T18:52:54.478Z] Starting server get operation
│ [2025-12-04T18:52:54.478Z] Polling server for value
│ [2025-12-04T18:52:54.478Z] Value found on server
│ [2025-12-04T18:52:54.479Z] Server get operation completed
│ Dan...
│ [2025-12-04T18:52:54.479Z] Starting server get operation
│ [2025-12-04T18:52:54.479Z] Polling server for value
│ [2025-12-04T18:52:54.479Z] Value found on server
│ [2025-12-04T18:52:54.479Z] Server get operation completed
│ Sending finalize packages to 3 participants...
│ Bob...
│ [2025-12-04T18:52:54.480Z] Starting server put operation
│ [2025-12-04T18:52:54.481Z] Sending PUT request to server
│ Carol...
│ [2025-12-04T18:52:54.481Z] Server put operation completed
│ [2025-12-04T18:52:54.481Z] Starting server put operation
│ [2025-12-04T18:52:54.481Z] Sending PUT request to server
│ [2025-12-04T18:52:54.482Z] Server put operation completed
│ Dan...
│ [2025-12-04T18:52:54.482Z] Starting server put operation
│ [2025-12-04T18:52:54.482Z] Sending PUT request to server
│ [2025-12-04T18:52:54.482Z] Server put operation completed
│ # Finalize preview for Bob
│
│ Collected 3 Round 2 responses to demo/alice/group-state/defe451bcbb099e6488201a36ba764dd4832880d8c4b339bb220fb4ce6c87ef4/collected_round2.json and sent 3 finalize requests.
│ ur:envelope/lftpsplrtpsotansfytansgshdcxdstdrsynrkhpsbiogmskfnfrztzttbtdvsdirycywtotfshfjztnetrpqzfradnyoycsielptpsotansfgjeiejeiofginjthsjzinknihoytpsotansfljzjpihjkjojljtjkihfpjpinietpsotansgshdcxolbzgumnfnlnstntyatdztssdsechlgltetnathfqdwtrlasskkgkntdaxeytsftoytpsotansfljnjpjlkpjtieeygdhsiajehsioihlftpsotaadamhdmukgcpisihhsieihjpcpftkgcpkoihjpjkinjljtcpftdydwcpiainjoisihjpjkkpinjyihcpftcpfggmgwgughdpfefyeyececehesdpgufdfpeceheydpkoehcpkidwcpjkiniojtinjtiohejkishsjpihcpftcpeyeceeideyenemhseeemieetdyemenecechsdyiaeohseeiaieeeiaeteeesemiaehdyhseheeeneeetdyiadyemeyesiaeyetiyiyeciyidesiheehsiaeheeecdyeccpkioytpsoiyjkihjtieihjptpsotanshdhdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisoytpsotansflihiojpjlkpjotpsotansgshdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkoytpsotansfljnjpjlkpjtieeygdhsiajehsioihlftpsotaadamhdmukgcpisihhsieihjpcpftkgcpkoihjpjkinjljtcpftdydwcpiainjoisihjpjkkpinjyihcpftcpfggmgwgughdpfefyeyececehesdpgufdfpeceheydpkoehcpkidwcpjkiniojtinjtiohejkishsjpihcpftcpeeeheseseteoeniyetehehhseciheoeciyehdydyeteoemiaeteyesihhseeeyetihdyiaiaidesehiheoeoiyhsecesiyeceeeeehetemieeoeciyeyidecesiydyihcpkioytpsoiyjkihjtieihjptpsotanshdhdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmoycsinlftpsotanshdhdcxykfmfzhkdyiyfsemurpfwyaotetbaspfdsvafzbtkbmntthffsbnctlrvyjzzsspoyaylstpsotansgylftanshfhdcxisioimuoprdsrewpgtweidlakibychfezccmrhmudwksecaafsdkqdqzbnuerklutansgrhdcxottbfdheievenlatfwisspwstbjoqzlpnemwpdlbsnwtsktbsetkrftivyiacxhhoycsfncsfgoycscstpsoihfpjziniaihoycsimlftansfwlrhdfptorovalutpkoatidtarlndnsloprregsflhswslueccysfcayncwfdceylwylkrdbybgollkzozsvojphdsonnhsoytpldoydkzmzcfrrtrddwehzshgjottietedsptcagsdsgtidpteedngomksblbaobsgdrkiyntfhloaeecadkefspltatyisdieehddatansfphdcxmelgprfnzeendngsdmdawlgwcmpmgochvomkgoylfwsktngocwsrfgmhgensflghoyahtpsotansgulftansfwlshddadeltlymkoymkeywyvskniedwhnkswdjtemtsktbkryhgcxykvensrliydlbepftpntzsgrvsvwgszmdneottrdinesiyaeieuyrlgdaaoelofecakpwzrerlutlnvwidhfkkectansgrhdcxzmkksktlbtldfhcxmngeasqdcmndctkinegsemwknsgrgmwyprcnstdnclsgsfdyoyaxtpsotansghhdfztelykgdstadttlcfkslfwffgvotpmozeaapdgsdidycytyjlndrkbthphgvwmtzmgyaadytteetooeflrhzskotyhgnbkkmsamjkldtkmoldpllawfldtiuetobzmtpswpwlgayn
│ {
│     request(ARID(26d2bff6)) [
│         'body': «"dkgFinalize"» [
│             ❰"group"❱: ARID(defe451b)
│             ❰"responseArid"❱: ARID(a615538e)
│             ❰"round2Package"❱: JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"signing_share":"254b267a47d807655a0c3a4cd4c8497c10a146480c0729c28ff5fb9e4ac14505"}) [
│                 "sender": XID(bbb3100d)
│             ]
│             ❰"round2Package"❱: JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"signing_share":"4199836f811a5e35f100837c829ea428e0ccb91e33fa59f544187d35f2b59f0e"}) [
│                 "sender": XID(cc871c6e)
│             ]
│         ]
│         'sender': XID(f53e4059) [
│             'key': PublicKeys(a3d556f1, SigningPublicKey(f53e4059, SchnorrPublicKey(6cd09ca0)), EncapsulationPublicKey(622f0841, X25519PublicKey(622f0841))) [
│                 'allow': 'All'
│                 'nickname': "Alice"
│             ]
│         ]
│         'senderContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Inspecting collected Round 2 packages

Collected Round 2 packages with each sender's next response ARID.

```
jq . demo/alice/group-state/*/collected_round2.json

│ {
│   "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│     "packages": {
│       "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "signing_share": "74baab4c2449001a795545d126b36ce3316302ad8a99adce63fa179a2e70cf00"
│       },
│       "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "signing_share": "ce1edcda3dc204a64b7609f50beeaaecca2e6a13b5ed913333cf59b568165808"
│       }
│     },
│     "response_arid": "ur:arid/hdcxpffztpkkaxgupllrntskkehflrjsbdlevllebdpewmtptifxmhbzdeaxpfleoefmmwjtsorp"
│   },
│   "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│     "packages": {
│       "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "signing_share": "254b267a47d807655a0c3a4cd4c8497c10a146480c0729c28ff5fb9e4ac14505"
│       },
│       "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "signing_share": "a3ead33eda52956077c52358d1a8e4aeb7fd26f9030233379506f7819277060b"
│       }
│     },
│     "response_arid": "ur:arid/hdcxhphygmktjkmohpbzvwaydkcpnsvtdkonehvednwfflrteoamvaaefwwegmlovwptrdwlkoox"
│   },
│   "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│     "packages": {
│       "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "signing_share": "4199836f811a5e35f100837c829ea428e0ccb91e33fa59f544187d35f2b59f0e"
│       },
│       "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "signing_share": "59aee18c9a4250b3b23ae9cd447abd7ea38f4854e352373892e90ee45d37aa04"
│       }
│     },
│     "response_arid": "ur:arid/hdcxbbrhlolooeplrtgaaahkmnmybzjspatbsktocahhjnghaybsldgwmdfxjogtvsdisghlzegl"
│   }
│ }
```

## Bob previews finalize response

Bob previews his finalize response structure (key packages) without posting.

```
BOB_GROUP_ID=$(jq -r '.groups | keys[0]' demo/bob/registry.json)
frost dkg participant finalize --preview --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_GROUP_ID}" | envelope format

│ ⬇️  ✅ Finalize request: 0s
│ {
│     response(ARID(26d2bff6)) [
│         'recipientContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│         'result': '' [
│             'isA': "dkgFinalizeResponse"
│             "group": ARID(defe451b)
│             "key_package": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"identifier":"0100000000000000000000000000000000000000000000000000000000000000","signing_share":"803a25a8d3c26128f2413e7698df1c7f89059bad9f464c2169334f5331412c0d","verifying_share":"564b7e3e1842baa0cf2ba8ddc99965cc8546127857e94484753584eb225bfe82","verifying_key":"5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3","min_signers":2})
│             "participant": XID(a84d0bec)
│             "public_key_package": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"verifying_shares":{"0100000000000000000000000000000000000000000000000000000000000000":"564b7e3e1842baa0cf2ba8ddc99965cc8546127857e94484753584eb225bfe82","0200000000000000000000000000000000000000000000000000000000000000":"dd766f4ee35466a3a006e91a43f225d3acc644ecd5163b5d2cba5ea6ef7e12a9","0300000000000000000000000000000000000000000000000000000000000000":"893ba94d539dcfde62c4d438bc51bcf92179ff05e8dee409874b4db4c7ad7da9"},"verifying_key":"5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3"})
│         ]
│         'sender': XID(a84d0bec) [
│             'key': PublicKeys(b98d0d37, SigningPublicKey(a84d0bec, SchnorrPublicKey(b6ad2f5b)), EncapsulationPublicKey(ee515e9d, X25519PublicKey(ee515e9d))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Bob posts finalize response

Bob posts his finalize response with generated key packages.

```
BOB_GROUP_ID=$(jq -r '.groups | keys[0]' demo/bob/registry.json)
frost dkg participant finalize --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_GROUP_ID}"

│ Fetching finalize request from Hubert...
│ [2025-12-04T18:52:54.520Z] Starting server get operation
│ [2025-12-04T18:52:54.520Z] Polling server for value
│ [2025-12-04T18:52:54.521Z] Value found on server
│ [2025-12-04T18:52:54.521Z] Server get operation completed
│ ⬇️  ✅ Finalize request: 0s
│ Received 2 Round 2 packages. Running DKG part3...
│ Generated key package and public key package.
│ [2025-12-04T18:52:54.522Z] Starting server put operation
│ [2025-12-04T18:52:54.522Z] Sending PUT request to server
│ [2025-12-04T18:52:54.523Z] Server put operation completed
│ ⬆️  ✅ Finalize Response: 0s
│ Posted finalize response to ur:arid/hdcxolbzgumnfnlnstntyatdztssdsechlgltetnathfqdwtrlasskkgkntdaxeytsftmedafgfe
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
```

## Carol posts finalize response

Carol posts her finalize response with generated key packages.

```
CAROL_GROUP_ID=$(jq -r '.groups | keys[0]' demo/carol/registry.json)
frost dkg participant finalize --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/carol/registry.json "${CAROL_GROUP_ID}"

│ Fetching finalize request from Hubert...
│ [2025-12-04T18:52:54.536Z] Starting server get operation
│ [2025-12-04T18:52:54.536Z] Polling server for value
│ [2025-12-04T18:52:54.537Z] Value found on server
│ [2025-12-04T18:52:54.537Z] Server get operation completed
│ ⬇️  ✅ Finalize request: 0s
│ Received 2 Round 2 packages. Running DKG part3...
│ Generated key package and public key package.
│ [2025-12-04T18:52:54.539Z] Starting server put operation
│ [2025-12-04T18:52:54.539Z] Sending PUT request to server
│ [2025-12-04T18:52:54.539Z] Server put operation completed
│ ⬆️  ✅ Finalize Response: 0s
│ Posted finalize response to ur:arid/hdcxlonbbtempdsogatbqzspkipeiytdbgfrbesobblfhpwsvwcmwnrhqzgmwmadgtfsvlfzaebz
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
```

## Dan posts finalize response

Dan posts his finalize response with generated key packages.

```
DAN_GROUP_ID=$(jq -r '.groups | keys[0]' demo/dan/registry.json)
frost dkg participant finalize --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/dan/registry.json "${DAN_GROUP_ID}"

│ Fetching finalize request from Hubert...
│ [2025-12-04T18:52:54.555Z] Starting server get operation
│ [2025-12-04T18:52:54.555Z] Polling server for value
│ [2025-12-04T18:52:54.556Z] Value found on server
│ [2025-12-04T18:52:54.556Z] Server get operation completed
│ ⬇️  ✅ Finalize request: 0s
│ Received 2 Round 2 packages. Running DKG part3...
│ Generated key package and public key package.
│ [2025-12-04T18:52:54.557Z] Starting server put operation
│ [2025-12-04T18:52:54.557Z] Sending PUT request to server
│ [2025-12-04T18:52:54.558Z] Server put operation completed
│ ⬆️  ✅ Finalize Response: 0s
│ Posted finalize response to ur:arid/hdcxsaloneidmhpdrhhnmwuevwntbgjsdwioadtalkmukbkpfehpglidbdidhndssfvdjpeeehzm
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
```

## Alice collects finalize responses

Alice fetches all finalize responses, validates them, saves collected key packages, and reports the group verifying key.

```
ALICE_GROUP_ID=$(jq -r '.groups | keys[0]' demo/alice/registry.json)
frost dkg coordinator finalize --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/alice/registry.json "${ALICE_GROUP_ID}"

│ Collecting finalize responses from 3 participants...
│ [2025-12-04T18:52:54.574Z] Starting server get operation
│ [2025-12-04T18:52:54.574Z] Polling server for value
│ [2025-12-04T18:52:54.575Z] Value found on server
│ [2025-12-04T18:52:54.575Z] Server get operation completed
│ ⬇️  ✅ Bob: 0s
│ [2025-12-04T18:52:54.575Z] Starting server get operation
│ [2025-12-04T18:52:54.575Z] Polling server for value
│ [2025-12-04T18:52:54.576Z] Value found on server
│ [2025-12-04T18:52:54.576Z] Server get operation completed
│ ⬇️  ✅ Carol: 0s
│ [2025-12-04T18:52:54.576Z] Starting server get operation
│ [2025-12-04T18:52:54.576Z] Polling server for value
│ [2025-12-04T18:52:54.577Z] Value found on server
│ [2025-12-04T18:52:54.577Z] Server get operation completed
│ ⬇️  ✅ Dan: 0s
│
│ Collected 3 finalize responses. Saved to /Users/wolf/Dropbox/DevProjects/BlockchainCommons/bc-rust/frost/demo/alice/group-state/defe451bcbb099e6488201a36ba764dd4832880d8c4b339bb220fb4ce6c87ef4/collected_finalize.json
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
```

## Inspecting collected finalize responses

Collected finalize responses keyed by participant XID.

```
jq . demo/alice/group-state/*/collected_finalize.json

│ {
│   "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│     "key_package": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "identifier": "0100000000000000000000000000000000000000000000000000000000000000",
│       "min_signers": 2,
│       "signing_share": "803a25a8d3c26128f2413e7698df1c7f89059bad9f464c2169334f5331412c0d",
│       "verifying_key": "5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3",
│       "verifying_share": "564b7e3e1842baa0cf2ba8ddc99965cc8546127857e94484753584eb225bfe82"
│     },
│     "public_key_package": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "verifying_key": "5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3",
│       "verifying_shares": {
│         "0100000000000000000000000000000000000000000000000000000000000000": "564b7e3e1842baa0cf2ba8ddc99965cc8546127857e94484753584eb225bfe82",
│         "0200000000000000000000000000000000000000000000000000000000000000": "dd766f4ee35466a3a006e91a43f225d3acc644ecd5163b5d2cba5ea6ef7e12a9",
│         "0300000000000000000000000000000000000000000000000000000000000000": "893ba94d539dcfde62c4d438bc51bcf92179ff05e8dee409874b4db4c7ad7da9"
│       }
│     }
│   },
│   "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│     "key_package": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "identifier": "0200000000000000000000000000000000000000000000000000000000000000",
│       "min_signers": 2,
│       "signing_share": "b1830ab64f211fb014795d713e66c17739c20122f6f092830862a00efbc39f0d",
│       "verifying_key": "5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3",
│       "verifying_share": "dd766f4ee35466a3a006e91a43f225d3acc644ecd5163b5d2cba5ea6ef7e12a9"
│     },
│     "public_key_package": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "verifying_key": "5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3",
│       "verifying_shares": {
│         "0100000000000000000000000000000000000000000000000000000000000000": "564b7e3e1842baa0cf2ba8ddc99965cc8546127857e94484753584eb225bfe82",
│         "0200000000000000000000000000000000000000000000000000000000000000": "dd766f4ee35466a3a006e91a43f225d3acc644ecd5163b5d2cba5ea6ef7e12a9",
│         "0300000000000000000000000000000000000000000000000000000000000000": "893ba94d539dcfde62c4d438bc51bcf92179ff05e8dee409874b4db4c7ad7da9"
│       }
│     }
│   },
│   "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│     "key_package": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "identifier": "0300000000000000000000000000000000000000000000000000000000000000",
│       "min_signers": 2,
│       "signing_share": "e2ccefc3cb7fdc3737b07c6ce4ec6570e97e68964c9bd9e5a790f1c9c446130e",
│       "verifying_key": "5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3",
│       "verifying_share": "893ba94d539dcfde62c4d438bc51bcf92179ff05e8dee409874b4db4c7ad7da9"
│     },
│     "public_key_package": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "verifying_key": "5f8de84b15d7ba8e8f956ff63a2f7c3baffba0e114147c62bb51a61d581c92f3",
│       "verifying_shares": {
│         "0100000000000000000000000000000000000000000000000000000000000000": "564b7e3e1842baa0cf2ba8ddc99965cc8546127857e94484753584eb225bfe82",
│         "0200000000000000000000000000000000000000000000000000000000000000": "dd766f4ee35466a3a006e91a43f225d3acc644ecd5163b5d2cba5ea6ef7e12a9",
│         "0300000000000000000000000000000000000000000000000000000000000000": "893ba94d539dcfde62c4d438bc51bcf92179ff05e8dee409874b4db4c7ad7da9"
│       }
│     }
│   }
│ }
```

## Verifying group key across all participants

Each registry records the same group verifying key (UR form).

```
for name in alice bob carol dan; do
  echo "$name:"
  jq -r '.groups | to_entries[0].value.verifying_key' demo/$name/registry.json
done

│ alice:
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
│ bob:
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
│ carol:
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
│ dan:
│ ur:signing-public-key/lfaohdcxhelgvsgrbztsrdmnmymdjlynftdlkefrpezonbvybbbbkeidrkgyolcahdcemowfchwpmdem
```

## Compose target envelope for signing

Build a sample target envelope with an assertion, wrap it for signing, and show its structure.

```
BASE_ENVELOPE=$(envelope subject type string "FROST signing demo target")
TARGET_ENVELOPE=$(echo "${BASE_ENVELOPE}" | envelope assertion add pred-obj string purpose string "threshold signing demo")
WRAPPED_TARGET=$(envelope subject type wrapped "${TARGET_ENVELOPE}")
echo "${WRAPPED_TARGET}" > demo/target-envelope.ur
envelope format < demo/target-envelope.ur

│ {
│     "FROST signing demo target" [
│         "purpose": "threshold signing demo"
│     ]
│ }
```

## Preview signInvite request (unsealed)

Preview the unsealed signInvite GSTP request (initial signing hop).

```
ALICE_GROUP_ID=$(jq -r '.groups | keys[0]' demo/alice/registry.json)
frost sign coordinator invite --preview --target demo/target-envelope.ur --registry demo/alice/registry.json "${ALICE_GROUP_ID}" | envelope format

│ {
│     request(ARID(3ac65a62)) [
│         'body': «"signInvite"» [
│             ❰"group"❱: ARID(defe451b)
│             ❰"minSigners"❱: 2
│             ❰"participant"❱: XID(a84d0bec) [
│                 "response_arid": ENCRYPTED [
│                     'hasRecipient': SealedMessage
│                 ]
│             ]
│             ❰"participant"❱: XID(bbb3100d) [
│                 "response_arid": ENCRYPTED [
│                     'hasRecipient': SealedMessage
│                 ]
│             ]
│             ❰"participant"❱: XID(cc871c6e) [
│                 "response_arid": ENCRYPTED [
│                     'hasRecipient': SealedMessage
│                 ]
│             ]
│             ❰"session"❱: ARID(3ac65a62)
│             ❰"target"❱: {
│                 "FROST signing demo target" [
│                     "purpose": "threshold signing demo"
│                 ]
│             }
│             ❰"validUntil"❱: 2025-12-04T19:52:54Z
│         ]
│         'date': 2025-12-04T18:52:54Z
│         'sender': XID(f53e4059) [
│             'key': PublicKeys(a3d556f1, SigningPublicKey(f53e4059, SchnorrPublicKey(6cd09ca0)), EncapsulationPublicKey(622f0841, X25519PublicKey(622f0841))) [
│                 'allow': 'All'
│                 'nickname': "Alice"
│             ]
│         ]
│         'senderContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Post signInvite request to Hubert

Coordinator posts the signInvite request to a single first-hop ARID (printed).

```
ALICE_GROUP_ID=$(jq -r '.groups | keys[0]' demo/alice/registry.json)
ALICE_SIGN_START_ARID=$(frost sign coordinator invite --verbose --storage $STORAGE --registry demo/alice/registry.json --target demo/target-envelope.ur "${ALICE_GROUP_ID}")
echo "${ALICE_SIGN_START_ARID}"

│ Posting signInvite request to ur:arid/hdcxuoehpmtikpfwhlatdesbgafshesawfbgosgswtktltsrbzcsetisoxgaimnlktntrpgetizo
│ ⬆️  ✅ Signing invite: 0s
│ [2025-12-04T18:52:54.654Z] Starting server put operation
│ [2025-12-04T18:52:54.654Z] Sending PUT request to server
│ [2025-12-04T18:52:54.655Z] Server put operation completed
│ ur:arid/hdcxuoehpmtikpfwhlatdesbgafshesawfbgosgswtktltsrbzcsetisoxgaimnlktntrpgetizo
```

## Bob inspects signInvite request

Bob fetches and decrypts the signInvite request via Hubert and views the details of the session.

```
START_PATH=$(ls -t demo/alice/group-state/*/signing/*/start.json | head -n1)
ALICE_SIGN_START_ARID=$(jq -r '.start_arid' "${START_PATH}")
BOB_SESSION_ID=$(frost sign participant receive --info --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${ALICE_SIGN_START_ARID}" | tee /dev/stderr | tail -n1)

│ ⬇️  ✅ Sign invite: 0s
│ Group: ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue
│ Coordinator: Alice
│ Min signers: 2
│ Participants: * Bob, Carol, Dan
│ Target:
│ {
│     "FROST signing demo target" [
│         "purpose": "threshold signing demo"
│     ]
│ }
│ ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd
```

## Carol inspects signInvite request

Carol fetches and decrypts the signInvite request via Hubert.

```
START_PATH=$(ls -t demo/alice/group-state/*/signing/*/start.json | head -n1)
ALICE_SIGN_START_ARID=$(jq -r '.start_arid' "${START_PATH}")
CAROL_SESSION_ID=$(frost sign participant receive --storage $STORAGE --timeout $TIMEOUT --registry demo/carol/registry.json "${ALICE_SIGN_START_ARID}" | tee /dev/stderr | tail -n1)

│ ⬇️  ✅ Sign invite: 0s
│ Group: ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue
│ Coordinator: Alice
│ Min signers: 2
│ Participants: Bob, * Carol, Dan
│ Target:
│ {
│     "FROST signing demo target" [
│         "purpose": "threshold signing demo"
│     ]
│ }
│ ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd
```

## Dan inspects signInvite request

Dan fetches and decrypts the signInvite request via Hubert.

```
START_PATH=$(ls -t demo/alice/group-state/*/signing/*/start.json | head -n1)
ALICE_SIGN_START_ARID=$(jq -r '.start_arid' "${START_PATH}")
DAN_SESSION_ID=$(frost sign participant receive --storage $STORAGE --timeout $TIMEOUT --registry demo/dan/registry.json "${ALICE_SIGN_START_ARID}" | tee /dev/stderr | tail -n1)

│ ⬇️  ✅ Sign invite: 0s
│ Group: ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue
│ Coordinator: Alice
│ Min signers: 2
│ Participants: Bob, Carol, * Dan
│ Target:
│ {
│     "FROST signing demo target" [
│         "purpose": "threshold signing demo"
│     ]
│ }
│ ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd
```

## Bob previews signInvite response

Bob dry-runs his signInvite response, showing commitments and next-hop response ARID without posting.

```
frost sign participant round1 --preview --registry demo/bob/registry.json "${BOB_SESSION_ID}" | envelope format

│ {
│     response(ARID(49c4ae56)) [
│         'recipientContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│         'result': '' [
│             'isA': "signRound1Response"
│             "commitments": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"hiding":"a995913c40cac29b9784ff3ebd85f827da91b6fadbbc70e761716136b96b798c","binding":"248cd56dc162a74cab4bf3998f36fd09e6465725bb91eb864406fb078ef44f56"})
│             "response_arid": ARID(9bf9d940)
│             "session": ARID(49c4ae56)
│         ]
│         'sender': XID(a84d0bec) [
│             'key': PublicKeys(b98d0d37, SigningPublicKey(a84d0bec, SchnorrPublicKey(b6ad2f5b)), EncapsulationPublicKey(ee515e9d, X25519PublicKey(ee515e9d))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Bob posts signInvite response

Bob posts his signInvite response to the coordinator.

```
frost sign participant round1 --verbose --storage $STORAGE --registry demo/bob/registry.json "${BOB_SESSION_ID}"

│ Posting signInvite response to ur:arid/hdcxbysobzktgtpmnlhhlruooektesvyweenfgkgdljemwztsplgbsprolfwpmhdoeckaolkeoin
│ [2025-12-04T18:52:54.754Z] Starting server put operation
│ [2025-12-04T18:52:54.754Z] Sending PUT request to server
│ [2025-12-04T18:52:54.755Z] Server put operation completed
│ ⬆️  ✅ Commitments: 0s
```

## Carol posts signInvite response

Carol posts her signInvite response to the coordinator.

```
frost sign participant round1 --storage $STORAGE --registry demo/carol/registry.json "${CAROL_SESSION_ID}"

│ ⬆️  ✅ Commitments: 0s
```

## Dan posts signInvite response

Dan posts his signInvite response to the coordinator.

```
frost sign participant round1 --storage $STORAGE --registry demo/dan/registry.json "${DAN_SESSION_ID}"

│ ⬆️  ✅ Commitments: 0s
```

## Alice collects commitments and posts signRound2 requests

Alice gathers the signInvite responses, aggregates commitments, sends per-participant signRound2 requests, and tells participants where to post their signature shares (share ARIDs).

```
START_PATH=$(ls -t demo/alice/group-state/*/signing/*/start.json | head -n1)
SESSION_ID=$(jq -r '.session_id' "${START_PATH}")
frost sign coordinator round1 --preview-share --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/alice/registry.json "${SESSION_ID}"

│ Collecting signInvite responses for session ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd from 3 participants...
│ [2025-12-04T18:52:54.797Z] Starting server get operation
│ [2025-12-04T18:52:54.797Z] Polling server for value
│ [2025-12-04T18:52:54.798Z] Value found on server
│ [2025-12-04T18:52:54.798Z] Server get operation completed
│ ⬇️  ✅ Bob: 0s
│ [2025-12-04T18:52:54.798Z] Starting server get operation
│ [2025-12-04T18:52:54.798Z] Polling server for value
│ [2025-12-04T18:52:54.799Z] Value found on server
│ [2025-12-04T18:52:54.799Z] Server get operation completed
│ ⬇️  ✅ Carol: 0s
│ [2025-12-04T18:52:54.799Z] Starting server get operation
│ [2025-12-04T18:52:54.799Z] Polling server for value
│ [2025-12-04T18:52:54.800Z] Value found on server
│ [2025-12-04T18:52:54.800Z] Server get operation completed
│ ⬇️  ✅ Dan: 0s
│ Dispatching signRound2 requests to 3 participants...
│ # signRound2 preview for ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl
│ {
│     request(ARID(49c4ae56)) [
│         'body': «"signRound2"» [
│             ❰"commitment"❱: XID(a84d0bec) [
│                 "commitments": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"hiding":"f095481f2a496cc392adc740b68cc12a02e659892d36e365049c99da754dce2a","binding":"b7546732bb750f14b23189e430adf188e69c69910afe06fdb058a5ab3f323509"})
│             ]
│             ❰"commitment"❱: XID(bbb3100d) [
│                 "commitments": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"hiding":"d8d88e823d5ff8a2bd2708f60eaf93a6b62a567dc7451d2fccc7cddceee3c88b","binding":"6bea7cc86c36c4ada609b3388c7953d8f4353a8a6d5a57f5568ea19ac99f7730"})
│             ]
│             ❰"commitment"❱: XID(cc871c6e) [
│                 "commitments": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"hiding":"6cb442c1f10eeb12529d4b3ca8eb95d423140c2e89aca4a39511d87964319af4","binding":"9949db018c7612796e5300532e763f454195eb712b18cae6b228ec80547a2704"})
│             ]
│             ❰"response_arid"❱: ARID(195f9831)
│             ❰"session"❱: ARID(49c4ae56)
│         ]
│         'sender': XID(f53e4059) [
│             'key': PublicKeys(a3d556f1, SigningPublicKey(f53e4059, SchnorrPublicKey(6cd09ca0)), EncapsulationPublicKey(622f0841, X25519PublicKey(622f0841))) [
│                 'allow': 'All'
│                 'nickname': "Alice"
│             ]
│         ]
│         'senderContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
│ [2025-12-04T18:52:54.801Z] Starting server put operation
│ [2025-12-04T18:52:54.801Z] Sending PUT request to server
│ [2025-12-04T18:52:54.802Z] Server put operation completed
│ ⬆️  ✅ Bob: 0s
│ [2025-12-04T18:52:54.802Z] Starting server put operation
│ [2025-12-04T18:52:54.802Z] Sending PUT request to server
│ [2025-12-04T18:52:54.802Z] Server put operation completed
│ ⬆️  ✅ Carol: 0s
│ [2025-12-04T18:52:54.803Z] Starting server put operation
│ [2025-12-04T18:52:54.803Z] Sending PUT request to server
│ [2025-12-04T18:52:54.803Z] Server put operation completed
│ ⬆️  ✅ Dan: 0s
│
│ Collected 3 signInvite responses. Saved to demo/alice/group-state/defe451bcbb099e6488201a36ba764dd4832880d8c4b339bb220fb4ce6c87ef4/signing/49c4ae56778206ef714b8a87c17bf5a63540ba55c81587eb3b2a76a19e7bbabb/commitments.json
│ Dispatched 3 signRound2 requests.
```

## Inspecting collected commitments

Commitments and ARIDs keyed by participant XID.

```
COMMITMENTS_PATH=$(ls -t demo/alice/group-state/*/signing/*/commitments.json | head -n1)
jq . "${COMMITMENTS_PATH}"

│ {
│   "commitments": {
│     "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│       "commitments": {
│         "binding": "b7546732bb750f14b23189e430adf188e69c69910afe06fdb058a5ab3f323509",
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "hiding": "f095481f2a496cc392adc740b68cc12a02e659892d36e365049c99da754dce2a"
│       },
│       "share_arid": "ur:arid/hdcxcfhemkehjepsvtdrcpfeiyaouefegwsahhpdpffmvohlleiseojegscamhcmsfghvalrknnd"
│     },
│     "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│       "commitments": {
│         "binding": "6bea7cc86c36c4ada609b3388c7953d8f4353a8a6d5a57f5568ea19ac99f7730",
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "hiding": "d8d88e823d5ff8a2bd2708f60eaf93a6b62a567dc7451d2fccc7cddceee3c88b"
│       },
│       "share_arid": "ur:arid/hdcxosgmwdghkemdfrwszebefxzelsesvajzykonosjyaorytolkglgyylhyaacpnycwvydkdidt"
│     },
│     "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│       "commitments": {
│         "binding": "9949db018c7612796e5300532e763f454195eb712b18cae6b228ec80547a2704",
│         "header": {
│           "ciphersuite": "FROST-ED25519-SHA512-v1",
│           "version": 0
│         },
│         "hiding": "6cb442c1f10eeb12529d4b3ca8eb95d423140c2e89aca4a39511d87964319af4"
│       },
│       "share_arid": "ur:arid/hdcxgdhnkndimofresvacmaaspkslbbtwyisatwsmnbdkkwfbnmdkohdgmataottfmdidmtstaah"
│     }
│   },
│   "group": "ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue",
│   "session": "ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd",
│   "target": "ur:envelope/tpsplftpsokscffggmgwgughcxjkiniojtinjtiocxieihjnjlcxjyhsjpioihjyoytpsoiojokpjpjojljkihtpsokojyisjpihjkisjljziecxjkiniojtinjtiocxieihjnjlglvwjoey"
│ }
```

## Bob previews signRound2Response

Bob fetches the signRound2 request, validates commitments, computes his signature share, and previews the response without posting.

```
frost sign participant round2 --preview --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_SESSION_ID}" | envelope format

│ ⬇️  ✅ signRound2 request: 0s
│ {
│     response(ARID(49c4ae56)) [
│         'recipientContinuation': ENCRYPTED [
│             'hasRecipient': SealedMessage
│         ]
│         'result': '' [
│             'isA': "signRound2Response"
│             "response_arid": ARID(d97ae74a)
│             "session": ARID(49c4ae56)
│             "signature_share": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"share":"d6b183def750b291b0e5443a77cf3c62e4b2dee13538fdbdda01c3efca427601"})
│         ]
│         'sender': XID(a84d0bec) [
│             'key': PublicKeys(b98d0d37, SigningPublicKey(a84d0bec, SchnorrPublicKey(b6ad2f5b)), EncapsulationPublicKey(ee515e9d, X25519PublicKey(ee515e9d))) [
│                 'allow': 'All'
│                 'nickname': "Bob"
│             ]
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
```

## Bob posts signRound2Response

Bob posts his signature share to the coordinator's share collection ARID.

```
frost sign participant round2 --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_SESSION_ID}"

│ Fetching signRound2 request from Hubert...
│ [2025-12-04T18:52:54.834Z] Starting server get operation
│ [2025-12-04T18:52:54.834Z] Polling server for value
│ [2025-12-04T18:52:54.834Z] Value found on server
│ [2025-12-04T18:52:54.834Z] Server get operation completed
│ ⬇️  ✅ signRound2 request: 0s
│ [2025-12-04T18:52:54.835Z] Starting server put operation
│ [2025-12-04T18:52:54.835Z] Sending PUT request to server
│ [2025-12-04T18:52:54.836Z] Server put operation completed
│ ⬆️  ✅ Signature Share: 0s
│ Posted signature share to ur:arid/hdcxcfhemkehjepsvtdrcpfeiyaouefegwsahhpdpffmvohlleiseojegscamhcmsfghvalrknnd
```

## Carol posts signRound2Response

Carol posts her signature share to the coordinator.

```
frost sign participant round2 --storage $STORAGE --timeout $TIMEOUT --registry demo/carol/registry.json "${CAROL_SESSION_ID}"

│ ⬇️  ✅ signRound2 request: 0s
│ ⬆️  ✅ Signature Share: 0s
```

## Dan posts signRound2Response

Dan posts his signature share to the coordinator.

```
frost sign participant round2 --storage $STORAGE --timeout $TIMEOUT --registry demo/dan/registry.json "${DAN_SESSION_ID}"

│ ⬇️  ✅ signRound2 request: 0s
│ ⬆️  ✅ Signature Share: 0s
```

## Inspecting Bob's signature share

Persisted signature share and signing context for Bob.

```
SHARE_PATH=$(ls -t demo/bob/group-state/*/signing/*/share.json | head -n1)
jq . "${SHARE_PATH}"

│ {
│   "commitments": {
│     "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│       "binding": "b7546732bb750f14b23189e430adf188e69c69910afe06fdb058a5ab3f323509",
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "hiding": "f095481f2a496cc392adc740b68cc12a02e659892d36e365049c99da754dce2a"
│     },
│     "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│       "binding": "6bea7cc86c36c4ada609b3388c7953d8f4353a8a6d5a57f5568ea19ac99f7730",
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "hiding": "d8d88e823d5ff8a2bd2708f60eaf93a6b62a567dc7451d2fccc7cddceee3c88b"
│     },
│     "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│       "binding": "9949db018c7612796e5300532e763f454195eb712b18cae6b228ec80547a2704",
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "hiding": "6cb442c1f10eeb12529d4b3ca8eb95d423140c2e89aca4a39511d87964319af4"
│     }
│   },
│   "finalize_arid": "ur:arid/hdcxbaihvwswtabgnlgtntgahetbiyktzerynsbwdrlfgmjscnswadcxmsmdqzbgluosinpmmkmk",
│   "response_arid": "ur:arid/hdcxcfhemkehjepsvtdrcpfeiyaouefegwsahhpdpffmvohlleiseojegscamhcmsfghvalrknnd",
│   "session": "ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd",
│   "signature_share": {
│     "header": {
│       "ciphersuite": "FROST-ED25519-SHA512-v1",
│       "version": 0
│     },
│     "share": "d6b183def750b291b0e5443a77cf3c62e4b2dee13538fdbdda01c3efca427601"
│   }
│ }
```

## Alice finalizes and posts finalize packages

Alice collects signature shares, aggregates the signature, and dispatches finalize packages; prints a preview of one finalize message.

```
START_PATH=$(ls -t demo/alice/group-state/*/signing/*/start.json | head -n1)
SESSION_ID=$(jq -r '.session_id' "${START_PATH}")
frost sign coordinator round2 --preview-finalize --verbose --storage $STORAGE --timeout $TIMEOUT --registry demo/alice/registry.json "${SESSION_ID}"

│ Collecting signature shares for session ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd from 3 participants...
│ [2025-12-04T18:52:54.888Z] Starting server get operation
│ [2025-12-04T18:52:54.888Z] Polling server for value
│ [2025-12-04T18:52:54.889Z] Value found on server
│ [2025-12-04T18:52:54.889Z] Server get operation completed
│ ⬇️  ✅ Bob: 0s
│ [2025-12-04T18:52:54.889Z] Starting server get operation
│ [2025-12-04T18:52:54.889Z] Polling server for value
│ [2025-12-04T18:52:54.889Z] Value found on server
│ [2025-12-04T18:52:54.889Z] Server get operation completed
│ ⬇️  ✅ Carol: 0s
│ [2025-12-04T18:52:54.889Z] Starting server get operation
│ [2025-12-04T18:52:54.889Z] Polling server for value
│ [2025-12-04T18:52:54.890Z] Value found on server
│ [2025-12-04T18:52:54.890Z] Server get operation completed
│ ⬇️  ✅ Dan: 0s
│
│ Aggregated signature for session ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd and prepared 3 finalize packages.
│ Signature verified against target and group key.
│ Dispatching finalize packages to 3 participants...
│ # signFinalize preview for ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo
│ {
│     event(ARID(49c4ae56)) [
│         'content': '' [
│             'isA': "signFinalize"
│             "session": ARID(49c4ae56)
│             "signature_share": XID(a84d0bec) [
│                 "share": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"share":"d6b183def750b291b0e5443a77cf3c62e4b2dee13538fdbdda01c3efca427601"})
│             ]
│             "signature_share": XID(bbb3100d) [
│                 "share": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"share":"a935c2399b0f3e0214d339afcc0ee45ee6800bacf605a29056e69bc3676ac80e"})
│             ]
│             "signature_share": XID(cc871c6e) [
│                 "share": JSON({"header":{"version":0,"ciphersuite":"FROST-ED25519-SHA512-v1"},"share":"7e2de8cef7fccfbd0d1f5386964db5919253a3bd855bce643e14dd197df2a303"})
│             ]
│         ]
│         'sender': XID(f53e4059) [
│             'key': PublicKeys(a3d556f1, SigningPublicKey(f53e4059, SchnorrPublicKey(6cd09ca0)), EncapsulationPublicKey(622f0841, X25519PublicKey(622f0841))) [
│                 'allow': 'All'
│                 'nickname': "Alice"
│             ]
│         ]
│     ]
│ } [
│     'signed': Signature
│ ]
│ [2025-12-04T18:52:54.891Z] Starting server put operation
│ [2025-12-04T18:52:54.891Z] Sending PUT request to server
│ [2025-12-04T18:52:54.892Z] Server put operation completed
│ ⬆️  ✅ Dan: 0s
│ [2025-12-04T18:52:54.892Z] Starting server put operation
│ [2025-12-04T18:52:54.892Z] Sending PUT request to server
│ [2025-12-04T18:52:54.892Z] Server put operation completed
│ ⬆️  ✅ Bob: 0s
│ [2025-12-04T18:52:54.892Z] Starting server put operation
│ [2025-12-04T18:52:54.892Z] Sending PUT request to server
│ [2025-12-04T18:52:54.893Z] Server put operation completed
│ ⬆️  ✅ Carol: 0s
│ ur:signature/lfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxehidhggh
│ ur:envelope/lftpsplftpsokscffggmgwgughcxjkiniojtinjtiocxieihjnjlcxjyhsjpioihjyoytpsoiojokpjpjojljkihtpsokojyisjpihjkisjljziecxjkiniojtinjtiocxieihjnjloyaxtpsotansghlfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxlfnncwdr
```

## Bob attaches aggregated signature

Bob fetches his finalize package, rebuilds the aggregated signature locally, attaches it to the target envelope, and formats the signed result.

```
BOB_ATTACH=$(frost sign participant finalize --storage $STORAGE --timeout $TIMEOUT --registry demo/bob/registry.json "${BOB_SESSION_ID}")
echo "${BOB_ATTACH}"
BOB_SIGNED=$(echo "${BOB_ATTACH}" | tail -n1)
echo "${BOB_SIGNED}" | envelope format

│ ⬇️  ✅ Finalize package: 0s
│ ur:signature/lfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxehidhggh
│ ur:envelope/lftpsplftpsokscffggmgwgughcxjkiniojtinjtiocxieihjnjlcxjyhsjpioihjyoytpsoiojokpjpjojljkihtpsokojyisjpihjkisjljziecxjkiniojtinjtiocxieihjnjloyaxtpsotansghlfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxlfnncwdr
│ {
│     "FROST signing demo target" [
│         "purpose": "threshold signing demo"
│     ]
│ } [
│     'signed': Signature(Ed25519)
│ ]
```

## Carol attaches aggregated signature

Carol retrieves her finalize package and attaches the verified group signature.

```
frost sign participant finalize --storage $STORAGE --timeout $TIMEOUT --registry demo/carol/registry.json "${CAROL_SESSION_ID}"

│ ⬇️  ✅ Finalize package: 0s
│ ur:signature/lfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxehidhggh
│ ur:envelope/lftpsplftpsokscffggmgwgughcxjkiniojtinjtiocxieihjnjlcxjyhsjpioihjyoytpsoiojokpjpjojljkihtpsokojyisjpihjkisjljziecxjkiniojtinjtiocxieihjnjloyaxtpsotansghlfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxlfnncwdr
```

## Dan attaches aggregated signature

Dan rebuilds and verifies the signature from his finalize package.

```
frost sign participant finalize --storage $STORAGE --timeout $TIMEOUT --registry demo/dan/registry.json "${DAN_SESSION_ID}"

│ ⬇️  ✅ Finalize package: 0s
│ ur:signature/lfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxehidhggh
│ ur:envelope/lftpsplftpsokscffggmgwgughcxjkiniojtinjtiocxieihjnjlcxjyhsjpioihjyoytpsoiojokpjpjojljkihtpsokojyisjpihjkisjljziecxjkiniojtinjtiocxieihjnjloyaxtpsotansghlfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxlfnncwdr
```

## Inspecting Bob's final signature state

Persisted aggregated signature, signature shares, and signed target UR for Bob.

```
FINAL_PATH=$(ls -t demo/bob/group-state/*/signing/*/final.json | head -n1)
jq . "${FINAL_PATH}"

│ {
│   "commitments": {
│     "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│       "binding": "b7546732bb750f14b23189e430adf188e69c69910afe06fdb058a5ab3f323509",
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "hiding": "f095481f2a496cc392adc740b68cc12a02e659892d36e365049c99da754dce2a"
│     },
│     "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│       "binding": "6bea7cc86c36c4ada609b3388c7953d8f4353a8a6d5a57f5568ea19ac99f7730",
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "hiding": "d8d88e823d5ff8a2bd2708f60eaf93a6b62a567dc7451d2fccc7cddceee3c88b"
│     },
│     "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│       "binding": "9949db018c7612796e5300532e763f454195eb712b18cae6b228ec80547a2704",
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "hiding": "6cb442c1f10eeb12529d4b3ca8eb95d423140c2e89aca4a39511d87964319af4"
│     }
│   },
│   "finalize_arid": "ur:arid/hdcxbaihvwswtabgnlgtntgahetbiyktzerynsbwdrlfgmjscnswadcxmsmdqzbgluosinpmmkmk",
│   "group": "ur:arid/hdcxuezefecwsbpfnlvafdlfadotjeosieutfdeylobtlkgreondprcxzogsvaspkbwkleonguue",
│   "session": "ur:arid/hdcxgassplhfktlfamwsjsgrleltsekgykolecfzrdgospbzltwmfrdrkooynnkgrdrkdavlgttd",
│   "signature": "ur:signature/lfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxehidhggh",
│   "signature_shares": {
│     "ur:xid/hdcxpdgtbdwpzmwplgaabbosfdzoihdiadmetkurbwsbprosetgeuyfspsdrlucertghollpzcvl": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "share": "d6b183def750b291b0e5443a77cf3c62e4b2dee13538fdbdda01c3efca427601"
│     },
│     "ur:xid/hdcxrkqdbebtnycmzemucfspemrtdafdaocnadlpaylbswguztkeswgyfxgozmiyvaisqdgddyjt": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "share": "a935c2399b0f3e0214d339afcc0ee45ee6800bacf605a29056e69bc3676ac80e"
│     },
│     "ur:xid/hdcxsfltcejtstqzayidseksectyhhntgwknjtkoykwerhmhihbwfxgmeokbnydmchdmvajywzjo": {
│       "header": {
│         "ciphersuite": "FROST-ED25519-SHA512-v1",
│         "version": 0
│       },
│       "share": "7e2de8cef7fccfbd0d1f5386964db5919253a3bd855bce643e14dd197df2a303"
│     }
│   },
│   "signed_target": "ur:envelope/lftpsplftpsokscffggmgwgughcxjkiniojtinjtiocxieihjnjlcxjyhsjpioihjyoytpsoiojokpjpjojljkihtpsokojyisjpihjkisjljziecxjkiniojtinjtiocxieihjnjloyaxtpsotansghlfaohdfzlnknknftoltsetynmkdkfhvsvocyuyssutfxjkgetbfzbzihgelsntdmdwykjltobefpetlejozspmytzofttnsfzoehylfshlltlggrprnljnqdjlztfrsnpenevoaxlfnncwdr"
│ }
```
