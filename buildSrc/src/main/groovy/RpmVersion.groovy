class RpmVersion {
    String version
    String originalRelease
    String release

    RpmVersion( String versionValue, String releaseValue) {
        setVersion(versionValue)
        setReleaseFromVersion(versionValue)
        originalRelease = releaseValue
    }

    String setVersion(versionValue) {
        if (versionValue != null && versionValue.contains("-")) {
            version = versionValue.substring(0, versionValue.indexOf('-'))
        } else {
            version = versionValue
        }
    }

    String setReleaseFromVersion(versionValue) {
        if (versionValue != null && versionValue.contains("-")) {
            release = versionValue.substring(versionValue.indexOf('-')+1, versionValue.length())
            release = release.replace('+', '.').replace("-", ".")
        } else {
            release = "1"
        }
    }

    String getVersion() {
        return version
    }

    String getRelease() {
        return release
    }

    String toString() {
        return version + '-' + release
    }
}

