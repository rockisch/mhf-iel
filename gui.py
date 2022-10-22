import subprocess
from functools import partial
import time
import tkinter as tk
from tkinter import ttk

import requests


LOGIN_ENDPOINT = 'http://127.0.0.1:8080'


class LoginScreen:
    def __init__(self, root):
        self.root = root
        self.username = tk.StringVar()
        self.password = tk.StringVar()
        self.error_label = None
        self.frm = ttk.Frame(root, padding=15)
        self.frm.grid(sticky=(tk.W, tk.N, tk.E, tk.S))

        input_frm = ttk.Frame(self.frm)
        input_frm.grid()
        ttk.Label(input_frm, text="Username").grid(row=1, column=1, pady=5, padx=5)
        ttk.Entry(input_frm, textvariable=self.username).grid(row=1, column=2)
        ttk.Label(input_frm, text="Password").grid(row=2, column=1, pady=5, padx=5)
        ttk.Entry(input_frm, textvariable=self.password).grid(row=2, column=2)

        button_frame = ttk.Frame(self.frm)
        button_frame.grid(pady=(10, 0))
        ttk.Button(button_frame, text="Login", command=self.login).grid(row=0, column=0)
        # ttk.Button(button_frame, text="Create Account", command=self.create_account).grid(row=0, column=1, padx=(20, 0), sticky=tk.E)

        for child in self.frm.winfo_children():
            child.grid_configure(sticky=(tk.W, tk.E))

    def login(self, *args):
        if self.error_label:
            self.error_label.destroy()

        try:
            resp = requests.post(LOGIN_ENDPOINT, json={'username': self.username.get(), 'password': self.password.get()})
        except Exception as e:
            self.error_label = ttk.Label(self.frm, text=f"Unable to connect to server: {e}")
            self.error_label.grid()
            return

        try:
            resp.raise_for_status()
        except requests.HTTPError as e:
            self.error_label = ttk.Label(self.frm, text=f"Unable to login: {resp.status_code}")
            self.error_label.grid()
            return

        data = resp.json()
        self.frm.destroy()

        CharSelectionScreen(self.root, data['characters'], data['token'])

    def create_account(self, *args):
        pass


class CharSelectionScreen:
    def __init__(self, root, characters=[], token=''):
        self.root = root
        self.token = token
        self.character_id = tk.IntVar()
        self.frm = ttk.Frame(root, padding=15)
        self.frm.grid(sticky=tk.W)

        char_frm = ttk.Frame(self.frm)
        char_frm.grid()
        for i, character in enumerate(characters, start=1):
            ttk.Label(char_frm, text=character['name']).grid(row=i, column=1)
            ttk.Button(char_frm, text='Select', command=partial(self.select, character['id'])).grid(row=i, column=2)

    def select(self, char_id, *args):
        print(str(char_id), self.token)
        subprocess.Popen(['./mhf-iel.exe', str(char_id), self.token])
        exit()

root = tk.Tk(className="mhf")
root.columnconfigure(0, weight=1)
root.rowconfigure(0, weight=1)
LoginScreen(root)
root.mainloop()
