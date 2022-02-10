package space.taran.arklib;

import java.nio.file.Path;

public class ArkLib {
    static {
        System.loadLibrary("arklib");
    }

    private static native long provideIndexNative(String rootPath);

    public static long provideIndex(Path rootPath) {
        return provideIndexNative(rootPath.toString());
    }
}
