import java.security.Signature
import java.security.KeyFactory
import java.security.MessageDigest
import java.security.spec.EdECPrivateKeySpec
import java.security.spec.NamedParameterSpec

import io.github.rctcwyvrn.blake3.Blake3

buildscript {
    dependencies {
        classpath("io.github.rctcwyvrn:blake3:1.3")
    }
}

plugins {
    id("base")
}

val moduleId:   String by rootProject.extra
val moduleName: String by rootProject.extra
val verName:    String by rootProject.extra
val verType:    String by rootProject.extra
val verCode:       Int by rootProject.extra
val verHash:    String by rootProject.extra

listOf(
    "debug",
    "release"
).forEach {
    val variantCapped = it.replaceFirstChar {
        if (it.isLowerCase()) it.titlecase() else it.toString()
    }
    val variantLowered = it.lowercase()
    val moduleDir = layout.buildDirectory.dir("outputs/module/${variantLowered}")
    val moduleDirFile = moduleDir.get().asFile
    val zipFileName = "${moduleName}-${verName}-${verCode}-${verHash}-${variantLowered}.zip".replace(' ', '-')

    val prepareModuleFilesTask = tasks.register<Copy>("prepareModuleFiles${variantCapped}") {
        group = "module"
        description = "Prepares module files for ${variantCapped}."

        dependsOn(
            ":fseeb:build${variantCapped}",
            ":fseep:assemble${variantCapped}",
            ":fseew:build${variantCapped}"
        )

        doFirst {
            with(moduleDirFile) {
                deleteRecursively()
            }
        }

        into(moduleDir)
            from("${projectDir}/src") {
                include(
                    "module.prop"
                )
                expand(
                    "moduleId" to "${moduleId}",
                    "moduleName" to "${moduleName}",
                    "versionName" to "${verName}${verType} (${verCode}-${verHash}-${variantLowered})",
                    "versionCode" to "${verCode}"
                )
            }
            from("${projectDir}/src") {
                exclude(
                    ".DS_Store",
                    "module.prop"
                )
            }
            from(rootProject.rootDir) {
                include(
                    "README.md",
                    "README4en-US.md"
                )
                rename(
                    "README.md",
                    "README4zh-Hans.md"
                )
            }
        into("bin") {
            from(project(":fseeb").file("target/aarch64-linux-android/${variantLowered}")) {
                include(
                    "fseec",
                    "fsees"
                )
            }
        }
        into("lib") {
            from(project(":fseeb").file("target/aarch64-linux-android/${variantLowered}")) {
                include("libutils.so")
            }
        }
        into("other") {
            from(moduleDir) {
                include(
                    "module.prop"
                )
                rename(
                    "module.prop",
                    "module.base"
                )
            }
            from(project(":fseep").file("build/outputs/apk/${variantLowered}")) {
                include(
                    "fseep-${variantLowered}.apk"
                )
                rename(
                    "fseep-${variantLowered}.apk",
                    "provider.apk"
                )
            }
        }
        into("webroot") {
            from(project(":fseew").file("dist")) {
                exclude(
                    ".DS_Store"
                )
            }
        }
    }

    val signModuleFilesTask = tasks.register("signModule${variantCapped}") {
        group = "module"
        description = "Sign module files for ${variantCapped}."

        dependsOn(prepareModuleFilesTask)

        val privateKeyFile = project.file("private_key")
        val publicKeyFile = project.file("public_key")
        doFirst {
            fun sha256Sum() {
                fileTree(moduleDir) {
                    exclude("MANIFEST")
                }.visit {
                    if (isDirectory) return@visit

                    val mdInstance = MessageDigest.getInstance("SHA3-256")
                    file.forEachBlock(4096) { bytes, size ->
                        mdInstance.update(bytes, 0, size)
                    }

                    val sha256File = File(moduleDirFile, "MANIFEST/${file.relativeTo(moduleDirFile)}.sha256")
                    sha256File.parentFile.mkdirs()

                    sha256File.writeText(
                        mdInstance.digest().joinToString("") {
                            "%02x".format(it)
                        }
                    )
                }
            }

            val raanaFile = File(moduleDirFile, "other/raana")
            if (privateKeyFile.exists()) {
                fun raanaSign() {
                    val BLAKE3Builder = StringBuilder()

                    listOf(
                        "bin/fseec",
                        "bin/fsees",
                        "lib/libutils.so",
                        "other/module.base",
                        "other/provider.apk",
                        "script/state.sh",
                        "script/util_functions.sh",
                        "action.sh",
                        "post-fs-data.sh",
                        "service.sh",
                        "uninstall.sh"
                    ).forEach {
                        println(it)

                        val mdInstance = Blake3.newInstance()
                        mdInstance.update(File(moduleDirFile, it))

                        BLAKE3Builder.append(
                            mdInstance.hexdigest()
                        )
                    }

                    val BLAKE3Hash = BLAKE3Builder.toString()

                    println(BLAKE3Hash)

                    val privateKeyBytes = privateKeyFile.readBytes()
                    val publicKeyBytes = publicKeyFile.readBytes()

                    val signInstance = Signature.getInstance("ed25519")
                    signInstance.initSign(
                        KeyFactory.getInstance("ed25519").generatePrivate(EdECPrivateKeySpec(NamedParameterSpec("ed25519"), privateKeyBytes))
                    )
                    signInstance.update(
                        BLAKE3Hash.toByteArray()
                    )

                    val finalSignBytes = signInstance.sign()

                    raanaFile.writeBytes(finalSignBytes.copyOfRange(0, 16))
                    raanaFile.appendBytes(publicKeyBytes.copyOfRange(0, 16))
                    raanaFile.appendBytes(finalSignBytes.copyOfRange(16, 48))
                    raanaFile.appendBytes(publicKeyBytes.copyOfRange(16, 32))
                    raanaFile.appendBytes(finalSignBytes.copyOfRange(48, 64))
                }

                raanaSign()

                sha256Sum()

                println("=== Forever Guard Cute Neko Raana ===")
            } else {
                println("No private_key found, This build will not be signed")

                raanaFile.createNewFile()

                sha256Sum()
            }
        }
    }

    val zipTask = tasks.register<Zip>("zip${variantCapped}") {
        group = "module"
        description = "Create module zip for ${variantCapped}."

        dependsOn(signModuleFilesTask)

        archiveFileName.set(zipFileName)
        destinationDirectory.set(layout.buildDirectory.file("outputs/${variantLowered}").get().asFile)
        from(moduleDir)
    }

    val pushTask = tasks.register<Exec>("push${variantCapped}") {
        group = "module"
        description = "Push ${variantLowered} module to device."

        dependsOn(zipTask)

        commandLine("adb", "push", zipTask.get().archiveFile.get().asFile, "/data/local/tmp")
    }

    tasks.register<Exec>("magisk${variantCapped}") {
        group = "module"
        description = "Installs ${variantLowered} module via Magisk."

        dependsOn(pushTask)

        commandLine("adb", "shell", "su", "-c", "magisk --install-module /data/local/tmp/${zipFileName}")
    }

    tasks.register<Exec>("ksud${variantCapped}") {
        group = "module"
        description = "Installs ${variantLowered} module via KernelSU."

        dependsOn(pushTask)

        commandLine("adb", "shell", "su", "-c", "ksud module install /data/local/tmp/${zipFileName}")
    }

    tasks.register<Exec>("apd${variantCapped}") {
        group = "module"
        description = "Installs ${variantLowered} module via APatch."

        dependsOn(pushTask)

        commandLine("adb", "shell", "su", "-c", "apd module install /data/local/tmp/${zipFileName}")
    }
}

tasks.register("zip") {
    group = "module"
    description = "Create module zip for Github Release."

    dependsOn(
        "zipDebug",
        "zipRelease"
    )
}