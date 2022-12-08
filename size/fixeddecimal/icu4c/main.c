// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "unicode/unumberformatter.h"
#include "unicode/usimplenumberformatter.h"
#include "unicode/uformattedvalue.h"
#include "unicode/utypes.h"
#include "unicode/uclean.h"
#include "unicode/ustring.h"
#include <string.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    UErrorCode status = U_ZERO_ERROR;

    u_init(&status);
    if (!U_SUCCESS(status))  {
        printf("Failed to initialize ICU4C\n");
        return status;
    }

    // UNumberFormatter* unumf = unumf_openForSkeletonAndLocale(u"", 0, "bn", &status);
    USimpleNumberFormatter* unumf = usnumf_openForLocale("bn", &status);
    if (!U_SUCCESS(status))  {
        printf("Failed to create UNumberFormatter\n");
        return status;
    }

    UFormattedNumber* result = unumf_openResult(&status);
    if (!U_SUCCESS(status))  {
        printf("Failed to create UFormattedNumber\n");
        return status;
    }

    // unumf_formatInt(unumf, 1000007, result, &status);
    usnumf_formatInt64(unumf, 1000007, result, &status);
    if (!U_SUCCESS(status))  {
        printf("Failed to format number\n");
        return status;
    }

    int32_t len_utf16 = 0;
    const UChar* result_utf16 = ufmtval_getString(unumf_resultAsValue(result, &status), &len_utf16, &status);
    if (!U_SUCCESS(status))  {
        printf("Failed to extract formatted number\n");
        return status;
    }

    int32_t len_utf8 = 0;
    char result_utf8[40] = {};
    u_strToUTF8(result_utf8, 40, &len_utf8, result_utf16, len_utf16, &status);
    if (!U_SUCCESS(status))  {
        printf("Failed to convert formatted number to UTF-8\n");
        return status;
    }

    printf("Output is %s\n", result_utf8);

    // unumf_close(unumf);
    usnumf_close(unumf);
    unumf_closeResult(result);

    return 0;
}
