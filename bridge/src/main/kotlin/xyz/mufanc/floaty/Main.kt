package xyz.mufanc.floaty

import android.graphics.PixelFormat
import android.graphics.Point
import android.os.ServiceManager
import android.view.Display
import android.view.IRotationWatcher
import android.view.IWindowManager
import android.view.SurfaceControl
import android.view.SurfaceControlTransactionHidden
import android.view.SurfaceHidden
import dev.rikka.tools.refine.Refine

object Main {

    private external fun render(surface: SurfaceHidden, densityScale: Float)

    @JvmStatic
    fun main(args: Array<String>) {
        System.load(args[0])
        val windows = IWindowManager.Stub.asInterface(ServiceManager.getService("window"))

        val initialSize = Point()
        windows.getInitialDisplaySize(Display.DEFAULT_DISPLAY, initialSize)
        val densityScale = windows.getBaseDisplayDensity(Display.DEFAULT_DISPLAY) / 160.0f

        val controller = SurfaceControl.Builder()
            .setName("floaty-egui")
            .setFormat(PixelFormat.RGBA_8888)
            .setBufferSize(initialSize.x, initialSize.y)
            .build()

        val rotation = windows.watchRotation(
            object : IRotationWatcher.Stub() {
                override fun onRotationChanged(rotation: Int) {
                    val size = rotatedSize(initialSize, rotation)
                    SurfaceControl.Transaction().use { transaction ->
                        transaction.setBufferSize(controller, size.x, size.y).apply()
                    }
                }
            },
            Display.DEFAULT_DISPLAY,
        )
        val size = rotatedSize(initialSize, rotation)

        SurfaceControl.Transaction().use { transaction ->
            transaction.setLayer(controller, Int.MAX_VALUE)
            transaction.setBufferSize(controller, size.x, size.y)

            Refine.unsafeCast<SurfaceControlTransactionHidden>(transaction).apply {
                setSkipScreenshot(controller, args[1].toBoolean())
                setTrustedOverlay(controller, true)
                show(controller)
            }

            transaction.apply()
        }

        val surface = SurfaceHidden(controller)
        try {
            render(surface, densityScale)
        } finally {
            surface.release()
        }
    }

    private fun rotatedSize(initialSize: Point, rotation: Int): Point {
        return if (rotation % 2 == 0) {
            Point(initialSize.x, initialSize.y)
        } else {
            Point(initialSize.y, initialSize.x)
        }
    }
}
