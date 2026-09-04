cmake_minimum_required(VERSION 3.16)

# 1. Project Metadata
project(encoder_example_project
    VERSION 1.0.0
    LANGUAGES C
)

# Set the C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# 2. Export compile_commands.json for clangd / IDEs
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# 3. Add Executable & Primary Source Files
add_executable(encoder_example_project
    src/main.c
)

# 4. Include Paths
target_include_directories(encoder_example_project
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_SOURCE_DIR}/vendor/sm_encoders/include
)

# 5. Pre-compiled Library Linking (sm_encoders)
# Find the static library in vendor/sm_encoders/lib
# MinGW builds name it "lib_sm_encoders.*"; MSVC builds drop the "lib" prefix
# and name it "_sm_encoders.*". Allow an empty prefix too (find_library always
# prepends CMAKE_FIND_LIBRARY_PREFIXES to NAMES, which defaults to "lib" only
# and would otherwise miss the MSVC-named file).
set(CMAKE_FIND_LIBRARY_PREFIXES "" "lib")
find_library(SM_ENCODERS_PATH
    NAMES lib_sm_encoders _sm_encoders
    PATHS ${CMAKE_CURRENT_SOURCE_DIR}/vendor/sm_encoders/lib
    NO_DEFAULT_PATH
)

if(SM_ENCODERS_PATH)
    target_link_libraries(encoder_example_project PRIVATE ${SM_ENCODERS_PATH})
else()
    message(FATAL_ERROR "Could not find sm_encoders static library in vendor/sm_encoders/lib")
endif()

# Rust's static std lib pulls in these system libraries on Windows.
if(WIN32)
    target_link_libraries(encoder_example_project PRIVATE ws2_32 userenv ntdll bcrypt advapi32)
endif()

# 6. Compiler Warnings
if(MSVC)
    target_compile_options(encoder_example_project PRIVATE /W4)
else()
    target_compile_options(encoder_example_project PRIVATE -Wall -Wextra)
endif()