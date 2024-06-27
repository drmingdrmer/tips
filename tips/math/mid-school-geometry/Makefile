all:
	find . -name "*.tex" -print0 | xargs -0 basename -s .tex -a  | xargs  -I '{}'  pdfcrop '{}.pdf' '{}-cropped.pdf'
	find . -name "*-cropped.pdf" -print0 | xargs -0 basename -s .pdf -a  | xargs  -I '{}'  convert -density 150 '{}.pdf' -quality 90 '{}.jpg'
