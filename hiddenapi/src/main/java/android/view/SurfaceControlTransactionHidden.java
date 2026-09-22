package android.view;

import dev.rikka.tools.refine.RefineAs;

@RefineAs(SurfaceControl.Transaction.class)
public class SurfaceControlTransactionHidden {
    public SurfaceControl.Transaction setSkipScreenshot(SurfaceControl control, boolean skip) {
        throw new RuntimeException("Stub!");
    }

    public SurfaceControl.Transaction setTrustedOverlay(SurfaceControl control, boolean trusted) {
        throw new RuntimeException("Stub!");
    }

    public SurfaceControl.Transaction show(SurfaceControl control) {
        throw new RuntimeException("Stub!");
    }
}
