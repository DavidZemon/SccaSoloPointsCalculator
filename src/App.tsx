import 'react-toastify/dist/ReactToastify.css';
import 'bootswatch/dist/slate/bootstrap.css';
import 'react-bootstrap-typeahead/css/Typeahead.css';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Col, Container, Row } from 'react-bootstrap';
import { toast, ToastContainer } from 'react-toastify';
import { ChampionshipType, SccaSoloPointsEngine } from 'rusty/rusty';
import { EventResults as EventResultsComponent } from './components/EventResults';
import { ChampionshipResults as ChampionshipResultsComponent } from './components/ChampionshipResults';
import { FileUploadBox } from './components/FileUploadBox';

interface AppState {
  eventResultsFile?: File;
  championshipResultsFiles: Partial<
    Record<keyof typeof ChampionshipType, File | undefined>
  >;

  processing: boolean;

  pointsEngine?: SccaSoloPointsEngine;
  driversInError?: string[];
  championshipResults?: Partial<Record<ChampionshipType, string>>;

  newLadies: string[];
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {
      championshipResultsFiles: {
        Class: undefined,
        PAX: undefined,
        Novice: undefined,
        Ladies: undefined,
      },
      processing: false,
      newLadies: [],
    };
  }

  render() {
    return (
      <div>
        <ToastContainer pauseOnHover />

        <Container fluid>
          {/* Page header */}
          <Row>
            <Col>
              <h1>SCCA Solo Points Calculator v2</h1>
            </Col>
          </Row>

          {/* File upload boxes */}
          <Row>
            <Col>
              <FileUploadBox
                label={'Full Event Results (by class)'}
                file={this.state.eventResultsFile}
                accept={'.csv'}
                onFileSelect={async (f) => {
                  try {
                    const rusty = new SccaSoloPointsEngine(
                      await f.text(),
                      false,
                    );
                    const driversInError =
                      rusty.js_drivers_in_error() as string[];
                    if (driversInError.length) {
                      console.error(`array=${JSON.stringify(driversInError)}`);
                      this.setState({ eventResultsFile: f, driversInError });
                    } else {
                      this.setState({
                        eventResultsFile: f,
                        pointsEngine: rusty,
                      });
                      await this.processChampionships();
                    }

                    return true;
                  } catch (e) {
                    console.error(e);
                    toast.error(
                      'File format does not match expected. Please export event results with raw times, grouped by class.',
                    );
                    return false;
                  }
                }}
                fileSelectedMessage={(f) => {
                  const elements = [
                    <p key={'resultsSummary'}>
                      Showing results for <code>{f.name}</code> as new event
                      results.
                    </p>,
                  ];
                  if (this.state.driversInError?.length) {
                    elements.push(
                      <p key={'errorIntro'}>
                        The following drivers appear to be be in an error state:
                      </p>,
                      <ul key={'errorList'}>
                        {this.state.driversInError.map((driver) => (
                          <li key={`driverInError-${driver}`}>{driver}</li>
                        ))}
                      </ul>,
                      <p key={'demandRefresh'}>
                        Please fix the errors and refresh this page.
                      </p>,
                      <p key={'fixInstructions'}>
                        To fix the errors, open TSAnnounce, search for each
                        class listed above, then re-run the export function from
                        TSAdmin
                      </p>,
                    );
                  }
                  return elements;
                }}
              />
            </Col>

            <Col>
              {Object.keys(this.state.championshipResultsFiles).map(
                (champTypeStr, index) => {
                  const championshipType =
                    champTypeStr as keyof typeof ChampionshipType;
                  return (
                    <FileUploadBox
                      key={index}
                      label={`${championshipType} Championship Standings`}
                      accept={'.xls,.xlsx'}
                      file={
                        this.state.championshipResultsFiles[championshipType]
                      }
                      onFileSelect={async (f) => {
                        await this.processChampionships({
                          championshipType,
                          newFile: f,
                        });
                        return true;
                      }}
                      fileSelectedMessage={(f) =>
                        this.state.eventResultsFile ? (
                          <p>
                            Showing <strong>{championshipType}</strong>{' '}
                            championship standings based on{' '}
                            <code>{f.name}</code>
                          </p>
                        ) : (
                          <p>
                            <code>{f.name}</code> set as{' '}
                            <strong>{championshipType}</strong> championship
                            standings. Please add event results to begin
                            processing.
                          </p>
                        )
                      }
                    />
                  );
                },
              )}
            </Col>
          </Row>

          {this.state.pointsEngine && (
            <EventResultsComponent pointsEngine={this.state.pointsEngine} />
          )}

          <ChampionshipResultsComponent
            results={this.state.championshipResults}
          />
        </Container>
      </div>
    );
  }

  private async processChampionships(params?: {
    championshipType: keyof typeof ChampionshipType;
    newFile: File;
  }): Promise<void> {
    if (params) {
      const { championshipType, newFile } = params;

      const mergedFiles = { ...this.state.championshipResultsFiles };
      mergedFiles[championshipType] = newFile;
      this.setState({ championshipResultsFiles: mergedFiles });

      if (this.state.pointsEngine) {
        this.setState({ processing: true });

        const resultsType = ChampionshipType[championshipType];
        const fileName = newFile.name;
        const newResults =
          this.state.pointsEngine.add_prior_championship_results(
            resultsType,
            new Uint8Array(await newFile.arrayBuffer()),
            fileName,
          );

        const championshipResults: Partial<
          Record<keyof typeof ChampionshipType, string>
        > = {
          ...this.state.championshipResults,
        };
        championshipResults[championshipType] = newResults;
        this.setState({
          processing: false,
          championshipResults,
        });
      }
    } else {
      await Promise.all(
        Object.entries(this.state.championshipResultsFiles).map(
          async ([championshipType, newFile]) => {
            if (championshipType && newFile) {
              await this.processChampionships({
                championshipType:
                  championshipType as keyof typeof ChampionshipType,
                newFile,
              });
            }
          },
        ),
      );
    }
  }
}

export default App;
