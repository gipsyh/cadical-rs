#include "internal.hpp"
namespace CaDiCaL {

// Private constructor.
File::File (Internal *i, bool w, int c, int p, FILE *f, const char *n)
    : internal (i),
#if !defined(QUIET) || !defined(NDEBUG)
      writing (w),
#endif
      close_file (c), child_pid (p), file (f), _name (strdup (n)),
      _lineno (1), _bytes (0) {
  (void) w;
  assert (f), assert (n);
}

FILE *File::open_file (Internal *internal, const char *path,
                       const char *mode) {
  (void) internal;
  return fopen (path, mode);
}
FILE *File::read_file (Internal *internal, const char *path) {
  MSG ("opening file to read '%s'", path);
  return open_file (internal, path, "r");
}
FILE *File::write_file (Internal *internal, const char *path) {
  MSG ("opening file to write '%s'", path);
  return open_file (internal, path, "wb");
}

File *File::read (Internal *internal, FILE *f, const char *n) {
  return new File (internal, false, 0, 0, f, n);
}
File *File::write (Internal *internal, FILE *f, const char *n) {
  return new File (internal, true, 0, 0, f, n);
}

File *File::read (Internal *internal, const char *path) {
  FILE *file;
  int close_input = 2;
  file = read_file (internal, path);
  close_input = 1;
  if (!file) return 0;
  return new File (internal, false, close_input, 0, file, path);
}

File *File::write (Internal *internal, const char *path) {
  FILE *file;
  int close_output = 3, child_pid = 0;
  file = write_file (internal, path), close_output = 1;
  if (!file)
    return 0;
  return new File (internal, true, close_output, child_pid, file, path);
}

void File::close (bool print) {
  assert (file);
  if (close_file == 0) {
    if (print)
      MSG ("disconnecting from '%s'", name ());
  }
  if (close_file == 1) {
    if (print)
      MSG ("closing file '%s'", name ());
    fclose (file);
  }
  if (close_file == 2) {
    if (print)
      MSG ("closing input pipe to read '%s'", name ());
    pclose (file);
  }
  file = 0; // mark as closed
}

bool File::piping () { return false; }
void File::flush () { assert (file); fflush (file); }
File::~File () { if (file) close (); free (_name); }

const char *version () { return "rIC3"; }
const char *copyright () { return "rIC3"; }
const char *authors () { return "rIC3"; }
const char *affiliations () { return "rIC3"; }
const char *signature () { return "rIC3"; }
const char *identifier () { return "rIC3"; }
const char *compiler () { return "rIC3"; }
const char *date () { return "rIC3"; }
const char *flags () { return "rIC3"; }
} // namespace CaDiCaL
