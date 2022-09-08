SET(PROJECT_NAME qiniu_sdk_test)
PROJECT(${PROJECT_NAME} LANGUAGES CXX)
CMAKE_MINIMUM_REQUIRED(VERSION 3.13)
SET(CMAKE_CXX_STANDARD 11)
SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -D__SOURCE_DIR__=\"\\\"${CMAKE_SOURCE_DIR}\\\"\"")

INCLUDE(FetchContent)
FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG release-1.12.1
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
SET(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

AUX_SOURCE_DIRECTORY(. DIR_SRCS_TEST)
ENABLE_TESTING()

INCLUDE_DIRECTORIES(..)
ADD_EXECUTABLE(qiniu_sdk_test ${DIR_SRCS_TEST})
TARGET_LINK_LIBRARIES(qiniu_sdk_test PRIVATE GTest::gtest_main PUBLIC qiniu_sdk_static)

INCLUDE(GoogleTest)
GTEST_DISCOVER_TESTS(qiniu_sdk_test)
