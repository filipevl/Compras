package com.filipevl.compras

import android.app.Application
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.launch
import uniffi.compras_core.ComprasCore
import java.io.File

class ComprasApplication : Application() {

    lateinit var core: ComprasCore
        private set

    override fun onCreate() {
        super.onCreate()

        val dbFile = File(filesDir, "compras.db")

        MainScope().launch(Dispatchers.IO) {
            core = ComprasCore.init(dbFile.absolutePath)
        }
    }
}