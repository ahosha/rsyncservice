package com.ibm.si.anchor.fvt

import com.aestasit.infrastructure.ssh.SshOptions
import com.aestasit.infrastructure.ssh.dsl.CommandOutput
import com.aestasit.infrastructure.ssh.dsl.SessionDelegate
import com.aestasit.infrastructure.ssh.dsl.SshDslEngine

import static groovy.lang.Closure.DELEGATE_FIRST

class SSHClient implements Executor {
    final static String user = "root"
    final static String key = "../provisioning/.keys/id_rsa"

    SshOptions options

        SSHClient(String host, int port) {
        options = new SshOptions()
        options.with {
            trustUnknownHosts = true
            defaultPort = port
            defaultHost = host
            defaultUser = user
            defaultKeyFile = new File(key)
            execOptions.with {
                failOnError = false
            }
        }
    }



    void remoteSession(@DelegatesTo(strategy = DELEGATE_FIRST,value = SessionDelegate) Closure cl) {
        new SshDslEngine(options).remoteSession(cl)
    }

    ExecResult exec(String command) {
        CommandOutput c
        remoteSession {
            c = exec command
        }
        new SSHResult(c)
    }
}
