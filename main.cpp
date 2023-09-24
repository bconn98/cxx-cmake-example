#include "cxx_bridge/lib.h"
#include <string>

int main()
{
    std::string lcLog = "This is my special C++ log";
    rust_part::log_me(lcLog);
}
