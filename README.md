# Crop Preserving Aspect Ratio

A simple tool to automatically crop artwork and restore it to the original aspect ratio afterwards. Primarily useful for
scanners which produce a scan with whitespace which, when removed, no longer results in the correct aspect ratio.

> **_NOTE:_**  Development build processes images extremely slowly. Please use release build.

Example usage:
```bash
# Process images in current directory and place them in ./out
cpar *.jpg out

# Whitespace detection controls
cpar *.jpg out -t 255 -p 0 # Only crop full white from edges of image
cpar *.jpg out -p 100      # Greedily crop image so no detected whitespace is left
cpar *.jpg out --ey 10     # Remove an additional 10px from detected bottom of image

# Blur output and downscale
cpar *.jpg out -b 1.5 -d 4.0
```

Help page:
```
Usage: cpar [OPTIONS] <SOURCE>... <OUTPUT>

Arguments:
  <SOURCE>...  Source file(s) to process
  <OUTPUT>     Output folder to place processed images within

Options:
  -t, --threshold <THRESHOLD>        Threshold value to identify as whitespace [default: 250]
      --x-threshold <X_THRESHOLD>    Threshold value in x-axis [aliases: --xt]
      --y-threshold <Y_THRESHOLD>    Threshold value in y-axis [aliases: --yt]
  -p, --percentile <PERCENTILE>      Percentage of rows/columns having crossed threshold to consider edge found [default: 95]
      --x-percentile <X_PERCENTILE>  Percentile in x-axis [aliases: --xp]
      --y-percentile <Y_PERCENTILE>  Percentile in y-axis [aliases: --yp]
  -e, --extra <EXTRA>                Extra margin to crop beyond found edge in both axes [default: 0]
      --x-extra <X_EXTRA>            Extra crop in x-axis [aliases: --ex]
      --y-extra <Y_EXTRA>            Extra crop in y-axis [aliases: --ey]
  -b, --blur <BLUR>                  Blur image by sigma
  -d, --downscale <DOWNSCALE>        Downscale image by factor [default: 1]
  -h, --help                         Print help
```