package com.ibm.si.anchor.fvt

import com.aestasit.infrastructure.ssh.dsl.CommandOutput

class SSHResult implements ExecResult  {
    String output
    int exitStatus

    SSHResult(CommandOutput result) {
        this.output = result.getOutput()
        this.exitStatus = result.getExitStatus()
    }

    String getOutput() {
        return output
    }
    int getExitCode() {
        return exitStatus
    }
}
