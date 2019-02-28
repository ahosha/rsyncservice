package com.ibm.si.anchor.fvt

class TestContext {
    final static String vsphereFlag = "../provisioning/vsphere_ip_box1.env"
    final static String vsphereFlag2 = "../provisioning/vsphere_ip_box2.env"
    final static int defaultSshPort = 22

    String sshIp
    String sshIp2
    int sshPort
    boolean isLocal

    TestContext() {
        def vsphF = new File(vsphereFlag)
        def vsphF2 = new File(vsphereFlag2)
        def isVsphere = vsphF.exists()
        def isVsphere2 = vsphF2.exists()

        if (!isVsphere2 && !isVsphere) {
            throw new Exception("cannot construct test context! no env file found")
        }


        if (isVsphere && isVsphere2) {
            def host = vsphF.readLines()
            def host2 = vsphF2.readLines()
            if (host.isEmpty() || host2.isEmpty()) {
                // cannot run tests on a remote vsphere vm if there is no ip!
                throw new Exception("cannot construct test context! virtual machine ip not found in vsphere env file")
            }
            this.sshIp = host.first().trim()
            this.sshIp2 = host2.first().trim()
            this.sshPort = defaultSshPort
        }
    }

    String getHost() {
        return sshIp
    }

    String getHost2() {
        return sshIp2
    }


    int getPort() {
        return sshPort
    }

    // run the test directly on the host system if true
    // run the test over ssh if false
    boolean isLocal() {
        return isLocal
    }


}
