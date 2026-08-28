-keep class com.xtrlumen.vbmeta.Entry
-assumenosideeffects class android.util.Log {
    public static int d(...);
}

-repackageclasses
-overloadaggressively
-allowaccessmodification