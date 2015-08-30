--- src/etc/mklldeps.py.orig	2015-08-30 18:28:29.025728000 +0200
+++ src/etc/mklldeps.py	2015-08-30 18:30:51.395744000 +0200
@@ -63,8 +63,8 @@
         lib = lib.strip()[1:]
     f.write("#[link(name = \"" + lib + "\"")
     # LLVM libraries are all static libraries
-    if 'LLVM' in lib:
-        f.write(", kind = \"static\"")
+    #if 'LLVM' in lib:
+    #    f.write(", kind = \"static\"")
     f.write(")]\n")
 
 # LLVM ldflags
