package com.ibm.si.anchor.fvt

class ShellClient implements Executor {
    ExecResult exec(String command) {
        Process proc = command.execute()
        proc.waitFor()
        new ShellResult(proc)
    }

}
