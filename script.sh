has_firefox=$(yabai -m query --windows | jq -r '.[] | select(.app == "Firefox Developer Edition") | .id')

if [[ $has_firefox ]]; then
  yabai -m window --focus $has_firefox
else
  has_open_space=$(yabai -m query --spaces | jq -r '.[] | select(.windows | length == 0) | .index')
  if [[ $has_open_space ]]; then
    echo "has open space"
    yabai -m space --focus $has_open_space
    open -a "Firefox Developer Edition"
    exit 0
  fi
  open -a "Firefox Developer Edition"
  sleep 2
  yabai -m space --create && \
    index=$(yabai -m query --spaces --display | jq 'map(.)[-1].index') && \
    yabai -m space --focus "$index"

  window_with_firefox=$(yabai -m query --windows | jq -r '.[] | select(.app == "Firefox Developer Edition") | .index')

  yabai -m space --swap recent
  # yabai -m window --focus $focus_to
fi

# yabai -m query --spaces | jq -r '.[] | select(.windows | length == 0) | .id'

# windows=$(yabai -m query --windows | jq -r '.[] | .id')

# last_window=$(yabai -m query --windows | jq -r '.[-1]')

# spaces_without_windows=$(yabai -m query --spaces | jq -r '.[] | select(.windows | length == 0) | .id')

# new_app_label="FIREFOX"

# app_name="Firefox"

# for app in $window_apps; do
#   if [[ $app == $app_name ]]; then
#     # goto firefox
#     echo "already have firefox"
#     exit 0
#   fi
# done

# # create a space named n and switch to it
# yabai -m space --create && \
#   index=$(yabai -m query --spaces --display | jq 'map(.)[-1].index') && \
#   yabai -m space --focus "$index" && \
#   yabai -m space --label "${new_app_label}" && \
#   /Applications/Firefox\ Developer\ Edition.app/Contents/MacOS/firefox --url about:newtab & && \

