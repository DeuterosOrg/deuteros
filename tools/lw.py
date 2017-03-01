#!/usr/bin/env python
# -*- coding: utf-8 -*-
#
# Copyright (c) 2017 Jakub Jermář
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions
# are met:
#
# - Redistributions of source code must retain the above copyright
#   notice, this list of conditions and the following disclaimer.
# - Redistributions in binary form must reproduce the above copyright
#   notice, this list of conditions and the following disclaimer in the
#   documentation and/or other materials provided with the distribution.
# - The name of the author may not be used to endorse or promote products
#   derived from this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
# IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
# OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
# IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
# INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
# NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
# DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
# THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
# THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#

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
    subprocess.call("ld -m elf_i386 --gc-sections -n -Tlink.ld %s" % cmdline,
        shell = True)

args = collect()
link(args)

