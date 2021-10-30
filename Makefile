PROJECT = toa
VPATH = $(wildcard [01]*)
.SUFFIXES: .tex .pdf
sources = $(wildcard [01]*/$(PROJECT)-*.tex)
targets = $(sources:.tex=.pdf)

.PHONY: deploy
default: commitid.tex $(targets)

commitid.tex:
	git log -1 --format="format:\gdef\\GITCommitHash{%H}%n\gdef\\GITCommitID{%h}%n\gdef\\GITAuthorDate{%aI}%n\gdef\\GITAuthorName{%an}%n" > commitid.tex	

.tex.pdf:
	cd $(dir $<); latexmk $(notdir $<)
