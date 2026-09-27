using System;
using System.Linq;
using System.Threading.Tasks;
using AltiumDesigner;

public class DRCResult
{
    public string Rule { get; set; }
    public string Severity { get; set; }
    public string Location { get; set; }
    public string Message { get; set; }
}

public class AltiumBridge
{
    private IApplication altiumApp;
    
    public void ImportIPC2581(string ipcPath)
    {
        altiumApp = new Application();
        var project = altiumApp.CreateNewProjectFromIPC(ipcPath);
        project.Save();
    }
    
    public async Task<DRCResult[]> RunDRC()
    {
        var drcRunner = new DesignRuleCheck();
        var results = await drcRunner.RunCheck();
        return results.Select(r => new DRCResult
        {
            Rule = r.Rule,
            Severity = r.Severity,
            Location = r.Location,
            Message = r.Message
        }).ToArray();
    }
}
