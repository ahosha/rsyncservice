package com.ibm.si.hardy.anchor

import com.ibm.si.anchor.fvt.ExecResult
import com.ibm.si.anchor.fvt.Executor
import com.ibm.si.anchor.fvt.SSHClient
import com.ibm.si.anchor.fvt.ShellClient
import com.ibm.si.anchor.fvt.TestContext

import spock.lang.Shared
import spock.lang.Specification

class AnchorSpecification extends Specification {


    @Shared
    protected Executor executor, executor_box2

// We need two executor to run commands on two different VM's

    void setupSpec() {
        TestContext context = new TestContext()
        if (context.isLocal()) {
            executor = new ShellClient()

        } else {
            executor = new SSHClient(context.getHost(), context.getPort())
            executor_box2 = new SSHClient(context.getHost2(), context.getPort())
        }
    }
    // delegate to an Executor to run an arbitrary command on the test host 1
    ExecResult execBox1(String command) {
        if (executor == null) {
            throw new Exception("Executor instance is uninitialized")
        }
        return executor.exec(command)
    }
// delegate to an Executor to run an arbitrary command on the test host 2
    ExecResult execBox2(String command) {
        if (executor_box2 == null) {
            throw new Exception("Executor instance is uninitialized")
        }
        return executor_box2.exec(command)
    }


}

