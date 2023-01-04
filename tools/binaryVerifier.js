import ora from 'ora';
import chalk from 'chalk';
import { exec } from 'child_process';
import fs from 'fs';
import crypto from 'crypto';

//Get from the command line
const [,, ...args] = process.argv;

console.log("args", args);