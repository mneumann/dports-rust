# Created by: Jean-Sébastien Pédron <dumbbell@FreeBSD.org>
# $FreeBSD$

PORTNAME=	rust-nightly
PORTVERSION=	1.3.0.20150703
CATEGORIES=	lang
MASTER_SITES=	http://www.ntecs.de/downloads/rust/:bootstrap
DISTFILES=	${RUST_BOOT}:bootstrap
EXTRACT_ONLY=	${DISTNAME}${_GITHUB_EXTRACT_SUFX} \
		${DISTFILE_compiler_rt} \
		${DISTFILE_hoedown} \
		${DISTFILE_jemalloc} \
		${DISTFILE_rust_installer}

USE_GITHUB=	yes
GH_ACCOUNT=	rust-lang
GH_PROJECT=	rust \
		compiler-rt:compiler_rt \
		hoedown:hoedown \
		jemalloc:jemalloc \
		rust-installer:rust_installer
GH_TAGNAME=	7b148381c7 \
		58ab642:compiler_rt \
		238c4d5:hoedown \
		e24a1a0:jemalloc \
		8e4f8ea:rust_installer

MAINTAINER=	dumbbell@FreeBSD.org
COMMENT=	Language with a focus on memory safety and concurrency

LICENSE=	APACHE20 \
		MIT
LICENSE_COMB=	dual
LICENSE_FILE=	${WRKSRC}/LICENSE-APACHE \
		${WRKSRC}/LICENSE-MIT

MAKE_JOBS_UNSAFE=yes
ONLY_FOR_ARCHS=	amd64
ONLY_FOR_ARCHS_REASON=	requires prebuilt bootstrap compiler

# FIXME: The bootstrapped rustc adds -L/usr/local/lib in front of
# the LDFLAGS. When stage0's rustc is linked, it picks the installed
# librust*so and fails.
CONFLICTS_BUILD=	rust \
			rust-nightly
CONFLICTS_INSTALL=	rust-nightly

RUST_BOOT=	rust-stage0-${RUST_BOOT_SIG}.tar.bz2
RUST_BOOT_SIG=  2015-06-12-50ab23d-dragonfly-x86_64-54171b3357723941a1025da60d090ca84634cdac

# Rust's libraries are named librustc_${component}-${RUST_VSN_HASH}.so.
# The hash depends on Rust version and channel. See
# $(CFG_FILENAME_EXTRA) definition in src/main.mk.
RUST_VSN=	${PORTVERSION:R}
RUST_CHANNEL=	${PORTNAME:S/^rust-//}
RUST_VSN_HASH!=	/usr/bin/printf '%s' ${RUST_VSN}-${RUST_CHANNEL} | /sbin/md5 -q | cut -c 1-8
PLIST_SUB+=	RUST_VSN_HASH=${RUST_VSN_HASH}
PLIST_SUB+=	RUST_TARGET=x86_64-unknown-${OPSYS:tl}

USES=		gmake python:2,build
HAS_CONFIGURE=	yes
CONFIGURE_ARGS=	--disable-valgrind --disable-docs \
		--enable-clang --mandir=${MANPREFIX}/man \
		--release-channel=${RUST_CHANNEL}

# Use LLVM from ports, instead of the copy shipped with rust.
LLVM_VER=	36
BUILD_DEPENDS+=	llvm${LLVM_VER}>=0:${PORTSDIR}/devel/llvm${LLVM_VER}
CONFIGURE_ARGS+=--llvm-root=${LOCALBASE}/llvm${LLVM_VER}

.if defined(BATCH) || defined(PACKAGE_BUILDING)
MAKE_ARGS+=	VERBOSE=1
.endif

OPTIONS_DEFINE=		LLNEXTGEN
LLNEXTGEN_DESC=		Build with grammar verification

LLNEXTGEN_BUILD_DEPENDS=	LLnextgen:${PORTSDIR}/devel/llnextgen

.include <bsd.port.options.mk>

.if ${OPSYS} == FreeBSD && ${OSVERSION} < 1000000
BROKEN=		Only compiles on FreeBSD 10 and 11
.endif

post-extract:
	@(${RMDIR} ${WRKSRC}/src/compiler-rt && \
	  ${MV} ${WRKSRC_compiler_rt} ${WRKSRC}/src/compiler-rt)
	@(${RMDIR} ${WRKSRC}/src/rt/hoedown && \
	  ${MV} ${WRKSRC_hoedown} ${WRKSRC}/src/rt/hoedown)
	@(${RMDIR} ${WRKSRC}/src/jemalloc && \
	  ${MV} ${WRKSRC_jemalloc} ${WRKSRC}/src/jemalloc)
	@(${RMDIR} ${WRKSRC}/src/rust-installer && \
	  ${MV} ${WRKSRC_rust_installer} ${WRKSRC}/src/rust-installer)
	@${MKDIR} ${WRKSRC}/dl
	${LN} -sf ${DISTDIR}/${RUST_BOOT} ${WRKSRC}/dl
	(cd ${WRKSRC} && find . -type d -exec chmod 0755 {} \;)

# In case the previous "make stage" failed, this ensures rust's
# install.sh won't backup previously staged files before reinstalling
# new ones. Otherwise, the staging directory is polluted with unneeded
# files.
pre-install:
	@if test -f ${STAGEDIR}${PREFIX}/lib/rustlib/manifest-rustc; then \
		${SED} -E -e 's,^(dir|file:),${STAGEDIR},' \
		< ${STAGEDIR}${PREFIX}/lib/rustlib/manifest-rustc \
		| ${XARGS} ${RM}; \
	fi
	@${RM} \
		${STAGEDIR}${PREFIX}/lib/rustlib/components \
		${STAGEDIR}${PREFIX}/lib/rustlib/manifest-rustc \
		${STAGEDIR}${PREFIX}/lib/rustlib/rust-installer-version \
		${STAGEDIR}${PREFIX}/lib/rustlib/uninstall.sh

post-install:
	@${RM} ${STAGEDIR}${PREFIX}/lib/rustlib/install.log
	@${REINPLACE_CMD} -e 's|${STAGEDIR}||' \
		${STAGEDIR}${PREFIX}/lib/rustlib/manifest-rustc
	@${RM} ${STAGEDIR}${PREFIX}/lib/rustlib/manifest-rustc.bak
	@${STRIP_CMD} \
		${STAGEDIR}${PREFIX}/bin/rustc \
		${STAGEDIR}${PREFIX}/bin/rustdoc \
		${STAGEDIR}${PREFIX}/lib/*.so \
		${STAGEDIR}${PREFIX}/lib/rustlib/*/lib/*.so
.if ${PORT_OPTIONS:MDOCS}
	@${MV} ${STAGEDIR}${PREFIX}/share/doc/rust ${STAGEDIR}${DOCSDIR}
.endif

.include <bsd.port.mk>
