#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
# Copyright 2017 Jakub Jermář
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

#
# This script collects the essential fragments of the linker script command
# line while throwing away the rest and then executes the actual linker with
# the collected arguments plus the arguments we want to use for linking the
# kernel.
#

"""
Linker wrapper for linking Deuteros
"""

import sys;
import subprocess;

def collect():

    cmd = []

    need_next = False

    for arg in sys.argv:
        if need_next:
            cmd = cmd + [arg]
            need_next = False
        elif arg == "-L" or arg == "-o":
            cmd = cmd + [arg]
            need_next = True
        elif arg[-2:] == ".o" or arg[-5:] == ".rlib":
            cmd = cmd + [arg]

    return cmd

def link(args):
    cmdline = " ".join(args)
    return subprocess.call(
            "ld -m elf_i386 --gc-sections -n -Ttools/link.ld %s" % cmdline,
            shell = True)

args = collect()
ret = link(args)
exit(ret)

