--- src/librustc_back/target/dragonfly_base.rs.orig
+++ src/librustc_back/target/dragonfly_base.rs
@@ -28,7 +28,7 @@ pub fn opts() -> TargetOptions {
             "-Wl,--as-needed".to_string(),
         ),
         position_independent_executables: true,
-        archive_format: "bsd".to_string(),
+        archive_format: "gnu".to_string(),
         exe_allocation_crate: super::best_allocator(),
         .. Default::default()
     }

