
        #define BUILD "1709356896-mozjpeg-sys"
        #ifndef INLINE
            #if defined(__GNUC__)
                #define INLINE inline __attribute__((always_inline))
            #elif defined(_MSC_VER)
                #define INLINE __forceinline
            #else
                #define INLINE inline
            #endif
        #endif
        #define FALLTHROUGH
        #define PACKAGE_NAME "mozjpeg-sys"
        #define VERSION "1.1.1"
        #define SIZEOF_SIZE_T 8
        