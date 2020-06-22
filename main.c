#include <stdio.h>
#include <stdlib.h>
#include <sys/sysctl.h>
#include <SystemConfiguration/SystemConfiguration.h>

// #include <SystemConfiguration/SCPreferencesSetSpecific.h>

char * MYCFStringCopyUTF8String(CFStringRef aString) {
  if (aString == NULL) {
    return NULL;
  }

  CFIndex length = CFStringGetLength(aString);
  CFIndex maxSize =
  CFStringGetMaximumSizeForEncoding(length, kCFStringEncodingUTF8) + 1;
  char *buffer = (char *)malloc(maxSize);
  if (CFStringGetCString(aString, buffer, maxSize,
                         kCFStringEncodingUTF8)) {
    return buffer;
  }
  free(buffer); // If we failed
  return NULL;
}

int main(int argc, char* argv[]) {
	CFStringRef cn = SCDynamicStoreCopyComputerName(NULL, NULL);

	char* string = MYCFStringCopyUTF8String(cn);

	printf("\"%s\"\n", string);

	/*char *p;
	size_t len;
	sysctlbyname("kern.ostype", NULL, &len, NULL, 0);
	p = malloc(len);
	sysctlbyname("kern.ostype", p, &len, NULL, 0);
	printf("%s (%ld)\n", p, len);*/
	return 0;
}
