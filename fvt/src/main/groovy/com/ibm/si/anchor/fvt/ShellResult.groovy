package com.ibm.si.anchor.fvt

class ShellResult implements ExecResult {
    String output
    int exitStatus

    ShellResult(Process result) {
        this.output = result.getText()
        this.exitStatus = result.exitValue()
    }

    String getOutput() {
        return output
    }

    int getExitCode() {
        return exitStatus
    }
}
