@0xac5d44cb6d8d6291;

struct CapMsg {
    channel @0 :Channel;
    command @1 :Command;

    # START
    startCmdCwd @2 :Text;
    startCmdArgv @3 :List(Text);
    startCmdDebugFlag @4 :Bool;
    startCmdMainJs @5 :Text;
    startCmdMainMap  @6 :Text;

    # CODE_FETCH
    codeFetchModuleSpecifier @7 :Text;
    codeFetchContainingFile @8 :Text;

    # CODE_FETCH_RES
    codeFetchResModuleName @9 :Text;
    codeFetchResFilename @10 :Text;
    codeFetchResSourceCode @11 :Text;
    codeFetchResOutputCode @12 :Text;

    # READ_FILE_SYNC
    readFileSyncFilename @13 :Text;
    readFileSyncData @14 :Data;

    enum Channel {
        start @0;
        os @1;
    }

    enum Command {
        startCmd @0;
        codeFetch @1;
        codeFetchRes @2;
        readFileSync @3;
        readFileSyncRes @4;
    }
}
