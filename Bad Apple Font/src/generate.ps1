Write-Output "Generating frames"
python generate_frames.py

Write-Output "Converting frames to SVG"
.\convert_to_svg.ps1

Write-Output "Generating font"
fontforge -lang=py -script generate_font.py

Write-Output "Generating file"
python generate_file.py
