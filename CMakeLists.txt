IF(POLICY CMP0048)
  CMAKE_POLICY(SET CMP0048 NEW)
ENDIF()

SET(PROJECT_NAME qiniu_sdk)
PROJECT(${PROJECT_NAME} VERSION 0.0.1 LANGUAGES CXX DESCRIPTION "Qiniu Rust SDK Bindings to C++" HOMEPAGE_URL "https://github.com/bachue/rust-sdk-cxx-bindings.git")
CMAKE_MINIMUM_REQUIRED(VERSION 3.19)
SET(CMAKE_CXX_STANDARD 11)

INCLUDE(FetchContent)
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
  GIT_TAG v0.2.2
)
FetchContent_MakeAvailable(Corrosion)
CORROSION_IMPORT_CRATE(MANIFEST_PATH rust/Cargo.toml)

IF(NOT DEFINED CMAKE_BUILD_TYPE OR CMAKE_BUILD_TYPE STREQUAL "")
  SET(config Debug)
ELSE()
  SET(config ${CMAKE_BUILD_TYPE})
ENDIF()

IF(CMAKE_VS_PLATFORM_NAME)
  SET(rust_sdk_ffi_build_dir "${CMAKE_VS_PLATFORM_NAME}/${config}")
ELSEIF(CMAKE_CONFIGURATION_TYPES)
  SET(rust_sdk_ffi_build_dir "${config}")
ELSE()
  SET(rust_sdk_ffi_build_dir .)
ENDIF()

SET(cargo_target_dir "${CMAKE_BINARY_DIR}/${rust_sdk_ffi_build_dir}/cargo/build")

INCLUDE_DIRECTORIES(includes ${cargo_target_dir}/${Rust_CARGO_TARGET_CACHED}/cxxbridge)
AUX_SOURCE_DIRECTORY(cpp CPP_SRCS)

ADD_LIBRARY(${PROJECT_NAME}_static STATIC ${CPP_SRCS})
ADD_LIBRARY(${PROJECT_NAME}_dynamic SHARED ${CPP_SRCS})

SET(OTHER_DEPS)

IF(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
  SET(OTHER_DEPS PRIVATE "-framework Security")
ENDIF()

TARGET_LINK_LIBRARIES(${PROJECT_NAME}_static PRIVATE qiniu_sdk_ffi ${OTHER_DEPS})
TARGET_LINK_LIBRARIES(${PROJECT_NAME}_dynamic PRIVATE qiniu_sdk_ffi ${OTHER_DEPS})

ADD_SUBDIRECTORY(gtests)
