using System;
using System.IO;
using System.Text;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using System.Security.Cryptography;
using System.Net;
using System.Threading;
using Altium.Sdk.PcbServer;
using Altium.Sdk.Pcb;
using Altium.Sdk.Core;

namespace OxideTech.AltiumIntegration
{
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct PhysicalDimension
    {
        public double width;
        public double length;
        public double wall_thickness;
        public double clearance;
    }

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct CollisionReport
    {
        [MarshalAs(UnmanagedType.U1)]
        public bool has_collision;
        public double overlap_distance;
        [MarshalAs(UnmanagedType.LPStr)]
        public string message;
    }

    public static class OxideCoreBindings
    {
        private const string DllPath = "oxide_core.dll";

        [DllImport(DllPath, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        public static extern IntPtr system_controller_new(PhysicalDimension dimension);

        [DllImport(DllPath, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool system_controller_run_gjk_clearance_solver(IntPtr controller, [MarshalAs(UnmanagedType.LPStr)] string designator, double x, double y, double height);

        [DllImport(DllPath, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        public static extern void system_controller_destroy(IntPtr controller);
    }

    /// <summary>
    /// Manages BYOK credentials storage using Windows Data Protection API (DPAPI).
    /// </summary>
    public static class SecureConfigStore
    {
        private static readonly string AppDataFolder = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData),
            "oxide_tech"
        );
        private static readonly string FilePath = Path.Combine(AppDataFolder, "credentials.bin");
        private static readonly byte[] Entropy = Encoding.UTF8.GetBytes("OxideTechEntropySalt");

        public static void SaveCredentials(string apiKey, string endpoint)
        {
            if (!Directory.Exists(AppDataFolder))
            {
                Directory.CreateDirectory(AppDataFolder);
            }

            string payload = $"{apiKey}||{endpoint}";
            byte[] rawBytes = Encoding.UTF8.GetBytes(payload);
            byte[] encryptedBytes = ProtectedData.Protect(rawBytes, Entropy, DataProtectionScope.CurrentUser);
            File.WriteAllBytes(FilePath, encryptedBytes);
        }

        public static (string ApiKey, string Endpoint) LoadCredentials()
        {
            if (!File.Exists(FilePath))
            {
                return (string.Empty, "https://generativelanguage.googleapis.com/v1beta/");
            }

            try
            {
                byte[] encryptedBytes = File.ReadAllBytes(FilePath);
                byte[] rawBytes = ProtectedData.Unprotect(encryptedBytes, Entropy, DataProtectionScope.CurrentUser);
                string payload = Encoding.UTF8.GetString(rawBytes);
                string[] parts = payload.Split(new[] { "||" }, StringSplitOptions.None);
                
                if (parts.Length >= 2)
                {
                    return (parts[0], parts[1]);
                }
                else if (parts.Length == 1)
                {
                    return (parts[0], "https://generativelanguage.googleapis.com/v1beta/");
                }
            }
            catch (Exception)
            {
                // Decryption failure or file corrupted
            }

            return (string.Empty, "https://generativelanguage.googleapis.com/v1beta/");
        }
    }

    /// <summary>
    /// BYOK settings manager dialog.
    /// </summary>
    public class ByokSettingsForm : Form
    {
        private TextBox _apiKeyTxt;
        private TextBox _endpointTxt;
        private Button _validateBtn;
        private Button _saveBtn;
        private Button _closeBtn;

        public ByokSettingsForm()
        {
            InitializeComponent();
            var creds = SecureConfigStore.LoadCredentials();
            _apiKeyTxt.Text = creds.ApiKey;
            _endpointTxt.Text = creds.Endpoint;
        }

        private void InitializeComponent()
        {
            this.Text = "Oxide-Tech BYOK Settings";
            this.Size = new System.Drawing.Size(460, 250);
            this.FormBorderStyle = FormBorderStyle.FixedDialog;
            this.MaximizeBox = false;
            this.MinimizeBox = false;
            this.StartPosition = FormStartPosition.CenterParent;

            var mainLayout = new TableLayoutPanel
            {
                Dock = DockStyle.Fill,
                ColumnCount = 2,
                RowCount = 4,
                Padding = new Padding(15)
            };
            mainLayout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 30F));
            mainLayout.ColumnStyles.Add(new ColumnStyle(SizeType.Percent, 70F));

            // Title
            var titleLbl = new Label
            {
                Text = "Gemini API Configuration",
                Font = new System.Drawing.Font("Segoe UI", 11F, System.Drawing.FontStyle.Bold),
                Height = 30,
                Dock = DockStyle.Fill
            };
            mainLayout.Controls.Add(titleLbl, 0, 0);
            mainLayout.SetColumnSpan(titleLbl, 2);

            // API Key
            var keyLbl = new Label { Text = "Gemini API Key:", Dock = DockStyle.Fill, TextAlign = System.Drawing.ContentAlignment.MiddleLeft };
            _apiKeyTxt = new TextBox { UseSystemPasswordChar = true, Dock = DockStyle.Fill };
            mainLayout.Controls.Add(keyLbl, 0, 1);
            mainLayout.Controls.Add(_apiKeyTxt, 1, 1);

            // Endpoint
            var endpointLbl = new Label { Text = "Endpoint:", Dock = DockStyle.Fill, TextAlign = System.Drawing.ContentAlignment.MiddleLeft };
            _endpointTxt = new TextBox { Dock = DockStyle.Fill };
            mainLayout.Controls.Add(endpointLbl, 0, 2);
            mainLayout.Controls.Add(_endpointTxt, 1, 2);

            // Buttons
            var btnPanel = new FlowLayoutPanel
            {
                Dock = DockStyle.Fill,
                FlowDirection = FlowDirection.RightToLeft,
                Padding = new Padding(0)
            };

            _closeBtn = new Button { Text = "Cancel" };
            _closeBtn.Click += (s, e) => this.Close();

            _saveBtn = new Button { Text = "Save Settings" };
            _saveBtn.Click += OnSave;

            _validateBtn = new Button { Text = "Validate Key" };
            _validateBtn.Click += OnValidate;

            btnPanel.Controls.Add(_closeBtn);
            btnPanel.Controls.Add(_saveBtn);
            btnPanel.Controls.Add(_validateBtn);

            mainLayout.Controls.Add(btnPanel, 0, 3);
            mainLayout.SetColumnSpan(btnPanel, 2);

            this.Controls.Add(mainLayout);
        }

        private void OnValidate(object sender, EventArgs e)
        {
            string key = _apiKeyTxt.Text.Trim();
            string endpoint = _endpointTxt.Text.Trim();

            if (string.IsNullOrEmpty(key))
            {
                MessageBox.Show("Please enter an API key.", "Oxide Tech", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                return;
            }

            _validateBtn.Enabled = false;
            try
            {
                string url = $"{endpoint.TrimEnd('/')}/models?key={key}";
                var request = (HttpWebRequest)WebRequest.Create(url);
                request.Method = "GET";
                request.Timeout = 5000;
                request.UserAgent = "Oxide-Tech-Altium-Validator";

                using (var response = (HttpWebResponse)request.GetResponse())
                {
                    if (response.StatusCode == HttpStatusCode.OK)
                    {
                        MessageBox.Show("API Key validation successful!", "Success", MessageBoxButtons.OK, MessageBoxIcon.Information);
                    }
                }
            }
            catch (WebException webEx)
            {
                string msg = webEx.Message;
                if (webEx.Response != null)
                {
                    using (var reader = new StreamReader(webEx.Response.GetResponseStream()))
                    {
                        string responseBody = reader.ReadToEnd();
                        msg += $"\n{responseBody}";
                    }
                }
                MessageBox.Show($"Validation failed: {msg}", "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Validation failed: {ex.Message}", "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
            finally
            {
                _validateBtn.Enabled = true;
            }
        }

        private void OnSave(object sender, EventArgs e)
        {
            string key = _apiKeyTxt.Text.Trim();
            string endpoint = _endpointTxt.Text.Trim();

            if (string.IsNullOrEmpty(key))
            {
                MessageBox.Show("API key cannot be empty.", "Oxide Tech", MessageBoxButtons.OK, MessageBoxIcon.Warning);
                return;
            }

            try
            {
                SecureConfigStore.SaveCredentials(key, endpoint);
                MessageBox.Show("Configuration stored securely via DPAPI.", "Success", MessageBoxButtons.OK, MessageBoxIcon.Information);
                this.DialogResult = DialogResult.OK;
                this.Close();
            }
            catch (Exception ex)
            {
                MessageBox.Show($"Failed to save credentials: {ex.Message}", "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }
    }

    /// <summary>
    /// Named pipes or local HTTP listener bridge that registers the C# clearance
    /// routine with the altium-mcp server scope.
    /// </summary>
    public class AltiumMcpBridge
    {
        private HttpListener _listener;
        private Thread _listenerThread;
        private readonly AltiumVerification _verification;
        private bool _isRunning;

        public AltiumMcpBridge(AltiumVerification verification)
        {
            _verification = verification;
        }

        public void Start(int port = 18501)
        {
            try
            {
                _listener = new HttpListener();
                _listener.Prefixes.Add($"http://127.0.0.1:{port}/altium/");
                _listener.Start();
                _isRunning = true;

                _listenerThread = new Thread(ListenLoop)
                {
                    IsBackground = true
                };
                _listenerThread.Start();
                Console.WriteLine($"Altium MCP IPC bridge started on port {port}");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Failed to start Altium MCP listener: {ex.Message}");
            }
        }

        public void Stop()
        {
            _isRunning = false;
            _listener?.Stop();
            _listener?.Close();
        }

        private void ListenLoop()
        {
            while (_isRunning)
            {
                try
                {
                    var context = _listener.GetContext();
                    var request = context.Request;
                    var response = context.Response;

                    if (request.HttpMethod == "POST" && request.Url.LocalPath.Contains("altium_run_clearance_audit"))
                    {
                        // Propagate key if sent, otherwise fallback to local DPAPI config
                        string apiKey = request.Headers["X-Gemini-Api-Key"];
                        if (string.IsNullOrEmpty(apiKey))
                        {
                            apiKey = SecureConfigStore.LoadCredentials().ApiKey;
                        }

                        // Run clearance audit
                        var report = _verification.RunClearanceSweep(apiKey);
                        
                        string jsonResponse = "{\"success\": " + (!report.has_collision).ToString().ToLower() +
                                              ", \"overlap\": " + report.overlap_distance.ToString() +
                                              ", \"message\": \"" + report.message + "\"}";
                        byte[] buffer = Encoding.UTF8.GetBytes(jsonResponse);

                        response.ContentType = "application/json";
                        response.ContentLength64 = buffer.Length;
                        response.OutputStream.Write(buffer, 0, buffer.Length);
                    }
                    else
                    {
                        response.StatusCode = (int)HttpStatusCode.NotFound;
                    }
                    response.OutputStream.Close();
                }
                catch (Exception)
                {
                    // Ignore errors during thread shutdown or request cancellation
                }
            }
        }
    }

    public class AltiumVerification
    {
        private const double CoordToMm = 2.54e-7;
        private AltiumMcpBridge _mcpBridge;

        public void InitializePlugin()
        {
            // Verify key on startup
            var creds = SecureConfigStore.LoadCredentials();
            if (string.IsNullOrEmpty(creds.ApiKey))
            {
                using (var settingsForm = new ByokSettingsForm())
                {
                    settingsForm.ShowDialog();
                }
            }

            // Start MCP IPC service
            _mcpBridge = new AltiumMcpBridge(this);
            _mcpBridge.Start();
        }

        public void ShutdownPlugin()
        {
            _mcpBridge?.Stop();
        }

        public void RunVerification()
        {
            var creds = SecureConfigStore.LoadCredentials();
            var report = RunClearanceSweep(creds.ApiKey);

            if (report.has_collision)
            {
                MessageBox.Show($"Verification complete. Detected clearance violations. {report.message}", "Oxide Tech", MessageBoxButtons.OK, MessageBoxIcon.Warning);
            }
            else
            {
                MessageBox.Show("Verification complete. No clearance violations detected.", "Oxide Tech", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
        }

        public CollisionReport RunClearanceSweep(string apiKey)
        {
            CollisionReport report = new CollisionReport
            {
                has_collision = false,
                overlap_distance = 0.0,
                message = "Verification succeeded."
            };

            IntPtr controllerPtr = IntPtr.Zero;
            try
            {
                IPCB_ServerInterface pcbServer = PCB.Server();
                if (pcbServer == null)
                {
                    report.has_collision = true;
                    report.message = "Altium PCB Server not available.";
                    return report;
                }

                IPCB_Board board = pcbServer.GetCurrentPCBBoard();
                if (board == null)
                {
                    report.has_collision = true;
                    report.message = "No active PCB document found.";
                    return report;
                }

                double boardWidthMm = board.BoardOutline.BoundingRectangle().Width * CoordToMm;
                double boardLengthMm = board.BoardOutline.BoundingRectangle().Height * CoordToMm;

                if (boardWidthMm <= 0 || boardLengthMm <= 0)
                {
                    report.has_collision = true;
                    report.message = "Invalid board outline dimensions.";
                    return report;
                }

                PhysicalDimension dim = new PhysicalDimension
                {
                    width = boardWidthMm,
                    length = boardLengthMm,
                    wall_thickness = 2.0,
                    clearance = 1.5
                };

                controllerPtr = OxideCoreBindings.system_controller_new(dim);

                if (controllerPtr == IntPtr.Zero)
                {
                    report.has_collision = true;
                    report.message = "Failed to initialize unmanaged Oxide System Controller.";
                    return report;
                }

                IPCB_BoardIterator iterator = board.BoardIterator_Create();
                iterator.AddFilter_ObjectSet(new TObjectSet(TObjectId.eComponentObject));
                iterator.AddFilter_LayerSet(LayerSet.AllLayers);
                iterator.AddFilter_Method(eProcessAll);

                int collisionCount = 0;
                IPCB_Component component = iterator.FirstPCBObject() as IPCB_Component;

                while (component != null)
                {
                    string designator = component.Name.Text;
                    double xMm = component.X * CoordToMm;
                    double yMm = component.Y * CoordToMm;
                    double heightMm = component.Height * CoordToMm;

                    bool collisionDetected = OxideCoreBindings.system_controller_run_gjk_clearance_solver(
                        controllerPtr, 
                        designator, 
                        xMm, 
                        yMm, 
                        heightMm
                    );
                    
                    if (collisionDetected)
                    {
                        collisionCount++;
                    }

                    component = iterator.NextPCBObject() as IPCB_Component;
                }
                
                board.BoardIterator_Destroy(ref iterator);

                if (collisionCount > 0)
                {
                    report.has_collision = true;
                    report.overlap_distance = 1.5; // threshold
                    report.message = $"Detected {collisionCount} clearance violations.";
                }
            }
            catch (Exception ex)
            {
                report.has_collision = true;
                report.message = $"Unexpected exception: {ex.Message}";
            }
            finally
            {
                if (controllerPtr != IntPtr.Zero)
                {
                    OxideCoreBindings.system_controller_destroy(controllerPtr);
                }
            }

            return report;
        }
    }
}
