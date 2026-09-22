package android.view;

import android.graphics.Point;
import android.os.Binder;
import android.os.IBinder;
import android.os.IInterface;

public interface IWindowManager extends IInterface {
    void getInitialDisplaySize(int displayId, Point size);
    int getBaseDisplayDensity(int displayId);
    int watchRotation(IRotationWatcher watcher, int displayId);

    abstract class Stub extends Binder implements IWindowManager {
        public static IWindowManager asInterface(IBinder binder) {
            throw new RuntimeException("Stub!");
        }
    }
}
